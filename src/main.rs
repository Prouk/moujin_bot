use ini::Ini;
use serenity::Client;
use serenity::prelude::GatewayIntents;
use songbird::SerenityInit;
mod handler;
mod bin;

#[tokio::main]
async fn main() {

    let i = Ini::load_from_file("./conf.ini").unwrap();

    let intents =
        GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_INTEGRATIONS
        | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(i.section(Some("TOKENS")).unwrap().get("DISCORD_TOKEN").unwrap(), intents)
        .event_handler(handler::main::get_handler())
        .register_songbird()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}