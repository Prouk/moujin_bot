use std::collections::HashMap;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::application_command::ApplicationCommandInteractionDataOptionValue;

pub async fn get_ff_char(command: &ApplicationCommandInteraction) -> String {
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

    // let params = [("name", name_req), ("world", world_req)];
    let mut map = HashMap::new();
    map.insert("name", &name_req);
    map.insert("world", &world_req);
    println!("sending : {:?}", &map);
    let res = reqwest::Client::new()
        .post("http://valentintahon.com/apiFfxiv")
        .json(&map)
        .send()
        .await.unwrap().text().await.unwrap().to_string();
    println!("got {:?}", res);
    return "```".to_owned()+&res+"```";
}