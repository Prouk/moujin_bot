use serde::Deserialize;
use serde::Serialize;
use std::string::String;
use serenity::client::Context;
use serenity::model::interactions::application_command::{ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue};
use serenity::model::interactions::InteractionResponseType;
use serenity::model::interactions::message_component::MessageComponentInteraction;
use serenity::utils::Colour;

#[derive(Serialize, Deserialize)]
struct FfChar {
    CharUrl: String,
    ImgUrl: String,
    Title: String,
    JobImg: String,
    Level: String,
    GrandCompany: String,
}

/// # Get the Final Fantasy XIV character infos embed through my api
/// ctx must be &context and command must be here to respond
pub async fn get_ff_char(ctx: &Context, command: &ApplicationCommandInteraction) {
    let world = command
        .data
        .options
        .get(0)
        .expect("Expected server name")
        .resolved
        .as_ref()
        .expect("Expected string option");

    let name = command
        .data
        .options
        .get(1)
        .expect("Expected character name")
        .resolved
        .as_ref()
        .expect("Expected string option");

    let world_req = if let ApplicationCommandInteractionDataOptionValue::String(world) = world
    {
        world.to_string()
    }else {
        "please format correctly the name".to_string()
    };


    let name_req = if let ApplicationCommandInteractionDataOptionValue::String(name) = name
    {
        name.to_string()
    }else {
        "please format correctly the name".to_string()
    };

    command.create_interaction_response(&ctx.http, |response|{
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message|message.content(format!("Searching : {} in {} ...", name_req, world_req)))


    }).await.map_err(|err| println!("${:?}",err)).ok();
    let htmlresp = reqwest::Client::new()
        .get("http://valentintahon.com/apiFfxiv?world=".to_owned()+&world_req+"&name="+name_req.replace(" ", "%2B").as_str())
        .send()
        .await.unwrap().text().await.unwrap().to_string();

    let json_resp: FfChar = serde_json::from_str(&*htmlresp).unwrap();

    if json_resp.Level == "" {
        command.edit_original_interaction_response(&ctx.http, |response|{
            response
                .content(format!("No character found for {} in {}", name_req, world_req))
        }).await.map_err(|err| println!("${:?}",err)).ok();
        return;
    }

    command.edit_original_interaction_response(&ctx.http, |response|{
        response
            .content("")
            .embed(|e|{
                e
                    .title(name_req)
                    .description(if !json_resp.Title.is_empty(){json_resp.Title}else{"No title set".to_string()})
                    .field("Level", json_resp.Level, false)
                    .field("Grand Company", if !json_resp.GrandCompany.is_empty(){json_resp.GrandCompany}else{"No Grand Company set".to_string()}, false)
                    .image(json_resp.ImgUrl)
                    .url(json_resp.CharUrl)
                    .thumbnail(json_resp.JobImg)
                    .color(Colour::BLURPLE)
            })
    }).await.map_err(|err| println!("{:?}",err)).ok();
}

/// ping pong
pub async fn ping(ctx: &Context, command: &ApplicationCommandInteraction) {
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(|message| message.content("`Mother Fu**ing Pong !`"))
        }).await.map_err(|err| println!("${:?}",err)).ok();
}

pub async fn no_command(ctx: &Context, command: &ApplicationCommandInteraction) {
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(|message| message.content("`No command found for this one.`"))
        }).await.map_err(|err| println!("${:?}",err)).ok();
}

pub async fn no_component_command(ctx: &Context, command: &MessageComponentInteraction) {
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(|message| message.content("`No command found for this one.`"))
        }).await.map_err(|err| println!("${:?}",err)).ok();
}