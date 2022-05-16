use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue};
use serenity::model::prelude::{InteractionResponseType};
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::utils::{Colour};

pub async fn join_voice(ctx: &Context, command: &ApplicationCommandInteraction) {
    let channel = command
        .data
        .options
        .get(0)
        .expect("Expected channel name")
        .resolved
        .as_ref()
        .expect("Expected string option");

    command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(format!("Trying to join channel ...")))
    }).await.map_err(|err| println!("${:?}",err)).ok();

    let choosen_channel = if let ApplicationCommandInteractionDataOptionValue::Channel(channel) = channel {
        channel
    }else {
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Wtf did you send me..."))
        }).await.map_err(|err| println!("${:?}",err)).ok();
        return;
    };

    let manager = songbird::get(&ctx)
        .await
        .expect("init").clone();

    let _handler = manager.join(command.guild_id.unwrap(), choosen_channel.id).await;
    command.edit_original_interaction_response(&ctx.http, |response| {
        response
            .content(format!("Joined {}", choosen_channel.name.as_ref().unwrap()))
    }).await.map_err(|err| println!("${:?}",err)).ok();
}

pub async fn play(ctx: &Context, command: &ApplicationCommandInteraction) {
    let url = command
        .data
        .options
        .get(0)
        .expect("Expected a url")
        .resolved
        .as_ref()
        .expect("Expected String option");

    let choosen_url = if let ApplicationCommandInteractionDataOptionValue::String(url) = url
    {
        url.to_string()
    } else {
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Wtf did you send me..."))
        }).await.map_err(|err| println!("${:?}",err)).ok();
        return;
    };

    if !choosen_url.starts_with("http") {
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Must provide http url"))
        }).await.map_err(|err| println!("${:?}",err)).ok();
        return;
    }

    let manager = songbird::get(ctx)
        .await
        .expect("init")
        .clone();

    if let Some(handler_lock) = manager.get(command.guild_id.unwrap()) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&choosen_url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content("Error sourcing ffmpeg"))
                }).await.map_err(|err| println!("${:?}", err)).ok();
                return;
            },
        };

        let title = source.metadata.title.as_ref().unwrap().clone();

        handler.enqueue_source(source);
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(format!("Added `{}` to queue", title)))
        }).await.map_err(|err| println!("${:?}", err)).ok();
    } else {
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Not in a voice channel"))
        }).await.map_err(|err| println!("${:?}", err)).ok();
    }
}

pub async fn player(ctx: &Context, command: &ApplicationCommandInteraction) {
    command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.embed(|e|
                e
                    .title("Moujin Player")
                    .description("Getting player infos")
                    .colour(Colour::RED)
            ))
    }).await.map_err(|err| println!("${:?}",err)).ok();
    create_current_music_embed(ctx, command).await;
}

pub async fn create_current_music_embed(ctx: &Context, command: &ApplicationCommandInteraction) -> Message {
    let manager = songbird::get(ctx)
        .await
        .expect("init")
        .clone();
    if let Some(handler_lock) = manager.get(command.guild_id.unwrap()) {
        let handler = handler_lock.lock().await;
        let metadata = handler.queue().current().unwrap().metadata().clone();
        let queue = handler.queue().current_queue();
        command.edit_original_interaction_response(&ctx.http, |response|
            response.embed(|e|
                e
                    .title("Moujin Player")
                    .description(metadata.title.unwrap())
                    .field("Channel", metadata.channel.unwrap(), false)
                    .field("time : ", metadata.duration.unwrap().as_secs().to_string()+" secs", true)
                    .url(metadata.source_url.unwrap())
                    .image(metadata.thumbnail.unwrap())
                    .colour(Colour::RED)
            )
                .components(|c|
                c
                    .create_action_row(|ar|
                        ar
                            .create_select_menu(|s|
                            {
                                s
                                    .placeholder("Music List")
                                    .options(|opts|
                                        {
                                            let mut i:u32 = 0;
                                            for track in queue {
                                                opts
                                                    .create_option(|opt|
                                                        opt
                                                            .label(track.metadata().title.as_ref().unwrap())
                                                            .description(track.metadata().artist.as_ref().unwrap())
                                                            .value(i)
                                                    );
                                                i = i+1;
                                            }
                                            opts
                                        }
                                    )
                                    .custom_id("musiclist")
                            }
                        )
                    )
                    .create_action_row(|ar|
                        ar
                            .create_button(|b|
                                b
                                    .style(ButtonStyle::Success)
                                    .label("pause / play")
                                    .custom_id("pp")
                            )
                            .create_button(|b|
                                b
                                    .style(ButtonStyle::Success)
                                    .label("next")
                                    .custom_id("next")
                            )
                            .create_button(|b|
                                b
                                    .style(ButtonStyle::Danger)
                                    .label("stop")
                                    .custom_id("stop")
                            )
                    )
                )
        ).await.unwrap()
    } else {
        command.edit_original_interaction_response(&ctx.http, |response|
            response.embed(|e|
                               e
                .title("Moujin Player")
                .description("No music playing...")
                .colour(Colour::RED)
            )
        ).await.unwrap()
    }
}

pub async fn skip(ctx: &Context, command: &ApplicationCommandInteraction) {
    let manager = songbird::get(ctx)
        .await
        .expect("init")
        .clone();
    if let Some(handler_lock) = manager.get(command.guild_id.unwrap()) {
        let handler = handler_lock.lock().await;
        handler.queue().skip();
        create_current_music_embed(ctx, command).await;
    }
}