use serenity::prelude::*;
use serenity::model::interactions::application_command::ApplicationCommand;


/// # Register music '/' commands
/// Preferably in 'ready' handler
pub async fn register_music_cmds(ctx: &Context) {
    ApplicationCommand::create_global_application_command(&ctx.http, |command| {
        command
            .name("play")
            .description("Add a music to the queue and play it")
    })
        .await.map_err(|err|println!("{:?}", err)).ok();
}