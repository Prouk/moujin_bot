use ini::Ini;
use serenity::client::Context;
use serenity::async_trait;
use serenity::model::id::MessageId;
use serenity::model::interactions::application_command::{ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue};
use serenity::model::prelude::{ChannelId, InteractionResponseType};
use serenity::model::interactions::message_component::{ButtonStyle, MessageComponentInteraction};
use serenity::utils::{Colour};
use songbird::tracks::{PlayMode};
use songbird::{Event, EventContext, TrackEvent, ytdl, EventHandler as VoiceEventHandler};
use songbird::id::GuildId;

pub async fn join_voice(ctx: &Context, command: &ApplicationCommandInteraction) {
    let channel = command
        .data
        .options
        .get(0)
        .expect("Expected channel name")
        .resolved
        .as_ref()
        .expect("Expected string option");

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

    let manager = songbird::get(ctx)
        .await
        .expect("init")
        .clone();

    let (_handle_lock, success) = manager.join(command.guild_id.unwrap(), choosen_channel.id).await;

    if let Ok(_channel) = success {
        command.create_interaction_response(&ctx, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .title("Joined")
                        .content(format!("Joined `{}`.",choosen_channel.name.as_ref().unwrap()))
                })
        }).await.map_err(|err| println!("${:?}",err)).ok();
    }
}

pub async fn play(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut is_playing: bool = false;
    let option = command
        .data
        .options
        .get(0)
        .expect("Expected url")
        .resolved
        .as_ref()
        .expect("Expected string option").clone();


    command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Adding the song to the queue ..."))
    }).await.map_err(|err| println!("${:?}",err)).ok();

    let url = if let ApplicationCommandInteractionDataOptionValue::String(option) = option {
        option
    }else {
        command.edit_original_interaction_response(&ctx.http, |response| {
            response
                .content("Wtf did you send me...")
        }).await.map_err(|err| println!("${:?}",err)).ok();
        return;
    };

    if !url.starts_with("http") {
        command.edit_original_interaction_response(&ctx.http, |response| {
            response
                .content("URL must be `http<rest of the url>`")
        }).await.map_err(|err| println!("${:?}",err)).ok();
        return;
    }

    let manager = songbird::get(ctx)
        .await
        .expect("init")
        .clone();

    if let Some(handler_lock) = manager.get(command.guild_id.unwrap()) {
        let mut handler = handler_lock.lock().await;

        let source = match ytdl(url).await {
            Ok(source) => source,
            Err(why) => {
                println!("{:?}",why);
                command.edit_original_interaction_response(&ctx.http, |response| {
                    response
                        .content("Error trying to add the song to queue.")
                }).await.map_err(|err| println!("${:?}",err)).ok();
                return;
            }
        };

        let play_state = match handler.queue().current() {
            Some(trackhandle) => trackhandle.get_info().await.unwrap().playing,
            None => PlayMode::Stop
        };

        if play_state == PlayMode::Play || play_state == PlayMode::Pause { is_playing = true; }

        handler.enqueue_source(source);

        let mut i = Ini::load_from_file("./conf.ini").unwrap();

        if is_playing {

            let command_id= i.section(Some(command.guild_id.unwrap().to_string())).unwrap().get("command_id").unwrap();

            let num = command_id.parse::<u64>().unwrap();

            command.channel_id.edit_message(&ctx.http, MessageId::from(num), |message|{
                message
                    .content(format!("`{}` added to the queue", handler.queue().current_queue().last().unwrap().metadata().title.as_ref().unwrap()))
                    .embed(|e|{
                        e
                            .colour(Colour::LIGHTER_GREY)
                            .title("Moujin Player")
                            .description(handler.queue().current().unwrap().metadata().title.as_ref().unwrap())
                            .image(handler.queue().current().unwrap().metadata().thumbnail.as_ref().unwrap())
                            .url(handler.queue().current().unwrap().metadata().source_url.as_ref().unwrap())
                            .field("Channel",handler.queue().current().unwrap().metadata().channel.as_ref().unwrap(), false)
                            .field("Coming Next",handler.queue().current_queue().get(1).unwrap().metadata().title.as_ref().unwrap(), false)
                    })
            }).await.map_err(|err| println!("${:?}",err)).ok();
        } else{

            let message = command.edit_original_interaction_response(&ctx.http, |response| {
                response
                    .content("")
                    .embed(|e|{
                        e
                            .colour(Colour::LIGHTER_GREY)
                            .title("Moujin Player")
                            .description(handler.queue().current().unwrap().metadata().title.as_ref().unwrap())
                            .image(handler.queue().current().unwrap().metadata().thumbnail.as_ref().unwrap())
                            .url(handler.queue().current().unwrap().metadata().source_url.as_ref().unwrap())
                            .field("Channel",handler.queue().current().unwrap().metadata().channel.as_ref().unwrap(), false)
                    })
                    .components(|components|{
                        components
                            .create_action_row(|car|{
                                car
                                    .create_button(|button|{
                                        button
                                            .style(ButtonStyle::Success)
                                            .label("Pause / Play")
                                            .custom_id("pause")
                                    })
                                    .create_button(|button|{
                                        button
                                            .style(ButtonStyle::Primary)
                                            .label("Skip")
                                            .custom_id("skip")
                                    })
                                    .create_button(|button|{
                                        button
                                            .style(ButtonStyle::Danger)
                                            .label("Stop")
                                            .custom_id("stop")
                                    })
                                    .create_button(|button|{
                                        button
                                            .style(ButtonStyle::Secondary)
                                            .label("Send me that")
                                            .custom_id("send_info")
                                    })

                            })
                    })
            }).await.unwrap();
            i.with_section(Some(command.guild_id.unwrap().to_string())).set("command_id", message.id.to_string());
            i.write_to_file("conf.ini").unwrap();

            handler.add_global_event(
                Event::Track(TrackEvent::End),
                TrackEndNotifier {
                    guild_id: GuildId::from(command.guild_id.unwrap()),
                    chan_id: command.channel_id,
                    cmd_ctx: ctx.clone()
                },
            );
        }
    }else{
        command.edit_original_interaction_response(&ctx.http, |response| {
            response
                .content("Not in a voice channel, try `/join <channel's name>` before adding a song to queue.")
        }).await.map_err(|err| println!("${:?}",err)).ok();
        return;
    }

}

pub async fn stop(ctx: &Context, command: &MessageComponentInteraction) {
    command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Stopping music..."))
    }).await.map_err(|err| println!("${:?}",err)).ok();

    let manager = songbird::get(ctx)
        .await
        .expect("init")
        .clone();

    if let Some(handler_lock) = manager.get(command.guild_id.unwrap()) {
        let mut handler = handler_lock.lock().await;
        handler.queue().stop();
        handler.leave().await.map_err(|err| println!("${:?}",err)).ok();

        let i = Ini::load_from_file("./conf.ini").unwrap();

        let command_id= i.section(Some(command.guild_id.unwrap().to_string())).unwrap().get("command_id").unwrap();

        let num = command_id.parse::<u64>().unwrap();

        command.channel_id.delete_message(&ctx.http,MessageId::from(num)).await.map_err(|err| println!("${:?}",err)).ok();
        command.edit_original_interaction_response(&ctx.http, |response| {
            response
                .content("Music player stopped.")
        }).await.map_err(|err| println!("${:?}",err)).ok();
        command.delete_original_interaction_response(&ctx.http).await.map_err(|err| println!("${:?}",err)).ok();

    }else {
        command.edit_original_interaction_response(&ctx.http, |response| {
            response
                .content("Not in a voice channel, try `/join <channel's name>` before stopping the player.")
        }).await.map_err(|err| println!("${:?}",err)).ok();
    }
}

pub async fn skip (ctx: &Context, command: &MessageComponentInteraction) {
    let manager = songbird::get(ctx)
        .await
        .expect("init")
        .clone();

    command.create_interaction_response(&ctx.http, |response|{
        response
            .kind(InteractionResponseType::UpdateMessage)
            .interaction_response_data(|message|{
                message
                    .content(format!("Skipping music..."))
            })
    }).await.map_err(|err| println!("${:?}",err)).ok();

    if let Some(handler_lock) = manager.get(command.guild_id.unwrap()) {
        let handler = handler_lock.lock().await;
        handler.queue().skip().map_err(|err| println!("${:?}",err)).ok();
        command.edit_original_interaction_response(&ctx.http,|message|message.content(format!("Song skipped."))).await.map_err(|err| println!("${:?}",err)).ok();
        command.delete_original_interaction_response(&ctx.http).await.map_err(|err| println!("${:?}",err)).ok();
    }else {
        command.edit_original_interaction_response(&ctx.http,|message|message.content(format!("No song playing, I don't even how you can click this."))).await.map_err(|err| println!("${:?}",err)).ok();
    }
}


pub async fn pause (ctx: &Context, command: &MessageComponentInteraction) {

    let manager = songbird::get(ctx)
        .await
        .expect("init")
        .clone();

    command.create_interaction_response(&ctx.http, |response|{
        response
            .kind(InteractionResponseType::UpdateMessage)
            .interaction_response_data(|message|{
                message
                    .content(format!("Pausing music ..."))
            })
    }).await.map_err(|err| println!("${:?}",err)).ok();

    if let Some(handler_lock) = manager.get(command.guild_id.unwrap()) {
        let handler = handler_lock.lock().await;

        let play_state = match handler.queue().current() {
            Some(trackhandle) => trackhandle.get_info().await.unwrap().playing,
            None => PlayMode::Stop
        };

        if play_state != PlayMode::Pause {
            handler.queue().pause().map_err(|err| println!("${:?}",err)).ok();
        }else {
            handler.queue().resume().map_err(|err| println!("${:?}",err)).ok();
        }
    }else {
        command.edit_original_interaction_response(&ctx.http,|message|message.content(format!("No song playing, I don't even how you can click this."))).await.map_err(|err| println!("${:?}",err)).ok();
    }

}

pub async fn send_music(ctx: &Context, command: &MessageComponentInteraction) {
    let manager = songbird::get(ctx)
        .await
        .expect("init")
        .clone();

    command.create_interaction_response(&ctx.http, |response|{
        response
            .kind(InteractionResponseType::UpdateMessage)
            .interaction_response_data(|message|{
                message
                    .content(format!("Sending you the music..."))
            })
    }).await.map_err(|err| println!("${:?}",err)).ok();

    if let Some(handler_lock) = manager.get(command.guild_id.unwrap()) {
        let handler = handler_lock.lock().await;

        command.user.direct_message(&ctx.http, |m| {
            m
                .content(format!("Here you go ma boi !"))
                .embed(|e| {
                    e
                        .colour(Colour::LIGHTER_GREY)
                        .title(handler.queue().current().unwrap().metadata().title.as_ref().unwrap())
                        .description(handler.queue().current().unwrap().metadata().date.as_ref().unwrap())
                        .image(handler.queue().current().unwrap().metadata().thumbnail.as_ref().unwrap())
                        .url(handler.queue().current().unwrap().metadata().source_url.as_ref().unwrap())
                        .field("Channel",handler.queue().current().unwrap().metadata().channel.as_ref().unwrap(), false)
                })
        }).await.map_err(|err| println!("${:?}",err)).ok();
    }
}

// EVENTS

struct TrackEndNotifier {
    guild_id: GuildId,
    chan_id: ChannelId,
    cmd_ctx: Context,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(_track_list) = ctx {
            let manager = songbird::get(&self.cmd_ctx)
                .await
                .expect("init")
                .clone();

            if let Some(handler_lock) = manager.get(self.guild_id) {
                let mut handler = handler_lock.lock().await;

                let i = Ini::load_from_file("./conf.ini").unwrap();

                let command_id= i.section(Some(self.guild_id.to_string())).unwrap().get("command_id").unwrap();

                let num = command_id.parse::<u64>().unwrap();

                if handler.queue().current().is_none() {
                    handler.queue().stop();
                    handler.leave().await.map_err(|err| println!("${:?}",err)).ok();
                    self.chan_id.delete_message(&self.cmd_ctx.http, MessageId::from(num)).await.map_err(|err| println!("${:?}",err)).ok();
                }else{
                    self.chan_id.edit_message(&self.cmd_ctx.http, MessageId::from(num), |message|{
                        message
                            .embed(|e|{
                                e
                                    .colour(Colour::LIGHTER_GREY)
                                    .title("Moujin Player")
                                    .description(handler.queue().current().unwrap().metadata().title.as_ref().unwrap())
                                    .image(handler.queue().current().unwrap().metadata().thumbnail.as_ref().unwrap())
                                    .url(handler.queue().current().unwrap().metadata().source_url.as_ref().unwrap())
                                    .field("Channel",handler.queue().current().unwrap().metadata().channel.as_ref().unwrap(), false);
                                if handler.queue().current_queue().len() > 1 {
                                    e.field("Coming Next",handler.queue().current_queue().get(1).unwrap().metadata().title.as_ref().unwrap(), false)
                                }else {
                                    e
                                }

                            })
                    }).await.map_err(|err| println!("${:?}",err)).ok();
                }
            }
        }
        None
    }
}