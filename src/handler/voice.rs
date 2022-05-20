use serenity::client::Context;
use serenity::model::interactions::application_command::{ApplicationCommand};
use serenity::model::prelude::application_command::ApplicationCommandOptionType;
use serenity::model::prelude::ChannelType;

/// # Register music '/' commands
/// Preferably in 'ready' handler
pub async fn register_music_cmds(ctx: &Context) {
    ApplicationCommand::create_global_application_command(&ctx.http, |command| {
        command
            .name("play")
            .description("Add a music to the queue and play it")
            .create_option(|option| {
                option
                    .name("url")
                    .description("the youtube url of the video")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    })
        .await.map_err(|err|println!("{:?}", err)).ok();


    ApplicationCommand::create_global_application_command(&ctx.http, |command| {
        command
            .name("join")
            .description("join a voice channel")
            .create_option(|option|{
                option
                    .name("channel")
                    .description("the channel to join")
                    .kind(ApplicationCommandOptionType::Channel)
                    .channel_types(&[ChannelType::Voice])
                    .required(true)
            })
    })
        .await.map_err(|err|println!("{:?}", err)).ok();
}