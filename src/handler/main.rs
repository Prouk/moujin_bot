use std::env;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::ApplicationCommand;

pub struct Handler;

/// # Get the music handler to pas to the bot app
pub fn get_handler() -> Handler {
    #[async_trait]
    impl EventHandler for Handler {
        async fn ready(&self, ctx: Context, _ready: Ready) {
            clean_cmds(&ctx).await;
            register_cmds(&ctx).await;
            crate::handler::music::register_music_cmds(&ctx).await;
        }
    }
    Handler
}

/// # Clean all olds commands registered on the bot account
async fn clean_cmds(ctx: &Context) {
    let guild_id = GuildId(
        env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse()
            .expect("GUILD_ID must be an integer"),
    );
    let mut commands = ApplicationCommand::get_global_application_commands(&ctx.http).await.unwrap();
    println!("Commands to clean : {}", commands.len());
    for command in commands {
        ApplicationCommand::delete_global_application_command(&ctx, command.id).await.map_err(|err|println!("{:?}",err)).ok();
        println!("Cleaned command : {}",command.name)
    }
}

/// # Register main '/' commands
/// Preferably in 'ready' handler
pub async fn register_cmds(ctx: &Context) {
    ApplicationCommand::create_global_application_command(&ctx.http, |command| {
        command.name("ping").description("An amazing command")
    })
        .await.map_err(|err|println!("{:?}", err)).ok();
}