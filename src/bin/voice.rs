use serenity::client::Context;
use serenity::model::channel::ChannelType;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::InteractionResponseType;

pub async fn join_voice(ctx: &Context, command: &ApplicationCommandInteraction) {
    let channel = command
        .data
        .options
        .get(0)
        .expect("Expected channel name")
        .resolved
        .as_ref()
        .expect("Expected string option");

    for (channel_id,channel) in command.guild_id.unwrap().channels(&ctx.http).await.unwrap() {
        if channel.kind != ChannelType::Voice {
            command.create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(channel.name+" is not a voice channel"))
            }).await.map_err(|err| println!("${:?}",err)).ok();
        }
    }
}