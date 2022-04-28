use std::env;
use serenity::Client;
use serenity::prelude::GatewayIntents;
mod handler;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_INTEGRATIONS
        | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(token, intents)
        .event_handler(handler::main::get_handler())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}