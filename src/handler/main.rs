use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::gateway::{Ready};
use serenity::model::interactions::application_command::ApplicationCommand;
use serenity::model::interactions::Interaction;
use serenity::model::prelude::application_command::ApplicationCommandOptionType;
use serenity::model::prelude::ChannelType;

use crate::bin;

pub struct Handler;

/// # Get the music handler to pas to the bot app
pub fn get_handler() -> Handler {
    #[async_trait]
    impl EventHandler for Handler {
        async fn ready(&self, ctx: Context, _ready: Ready) {
            // clean_cmds(&ctx).await;
            register_cmds(&ctx).await;
            crate::handler::music::register_music_cmds(&ctx).await;
        }
        async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
            if let Interaction::ApplicationCommand(command) = interaction {
                match command.data.name.as_str() {
                    "ping" => {
                        bin::main::ping(&ctx,&command).await
                    },
                    "character" => {
                        bin::main::get_ff_char(&ctx,&command).await
                    },
                    _ => {
                        bin::main::no_command(&ctx,&command).await
                    },
                };
            }
        }
    }
    Handler
}

/// # Register main '/' commands
/// Preferably in 'ready' handler
pub async fn register_cmds(ctx: &Context) {
    ApplicationCommand::create_global_application_command(&ctx.http, |command| {
        command
            .name("ping")
            .description("An amazing command")
    })
        .await.map_err(|err|println!("{:?}", err)).ok();
    ApplicationCommand::create_global_application_command(&ctx.http, |command| {
        command
            .name("character")
            .description("Get the datas of the character in the desired DataCenter")
            .create_option(|option| {
                option
                    .name("datacenter")
                    .description("the datacenter the character's datas will be pulled of")
                    .kind(ApplicationCommandOptionType::String)
                    .add_string_choice("Moogle, the best", "Moogle")
                    .add_string_choice("Louisoix", "Louisoix")
                    .add_string_choice("Spriggan", "Spriggan")
                    .add_string_choice("Ragnarok", "Ragnarok")
                    .add_string_choice("Omega", "Omega")
                    .add_string_choice("Cerberus", "Cerberus")
                    .add_string_choice("Lich", "Lich")
                    .add_string_choice("Odin", "Odin")
                    .add_string_choice("Phoenix", "Phoenix")
                    .add_string_choice("Shiva", "Shiva")
                    .add_string_choice("Twintania", "Twintania")
                    .add_string_choice("Zodiark", "Zodiark")
                    .add_string_choice("Bismarck", "Bismarck")
                    .add_string_choice("Ravana", "Ravana")
                    .add_string_choice("Sephirot", "Sephirot")
                    .add_string_choice("Sophia", "Sophia")
                    .add_string_choice("Zurvan", "Zurvan")
                    .add_string_choice("Aegis", "Aegis")
                    .add_string_choice("Atomos", "Atomos")
                    .add_string_choice("Carbuncle", "Carbuncle")
                    .add_string_choice("Garuda", "Garuda")
                    .add_string_choice("Gungnir", "Gungnir")
                    .add_string_choice("Kujata", "Kujata")
                    .add_string_choice("Ramuh", "Ramuh")
                    .add_string_choice("Tonberry", "Tonberry")
                    .add_string_choice("Typhon", "Typhon")
                    .add_string_choice("Unicorn", "Unicorn")
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("name")
                    .description("character's firstname and lastname")
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

/// # Clean all olds commands registered on the bot account
async fn clean_cmds(ctx: &Context) {
    let commands = ApplicationCommand::get_global_application_commands(&ctx.http).await.unwrap();
    println!("Commands to clean : {}", commands.len());
    for command in commands {
        ApplicationCommand::delete_global_application_command(&ctx, command.id).await.map_err(|err|println!("{:?}",err)).ok();
        println!("Cleaned command : {}",command.name)
    }
}