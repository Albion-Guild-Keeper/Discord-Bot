use crate::models::user::User;
use serde_json::json;
use serenity::model::prelude::Member;
use serenity::prelude::*;

// Gestisce l'evento GuildMemberAdd (quando un utente si unisce a un server).
pub async fn handle_guild_member_add(_ctx: Context, new_member: Member) {
    // Trova il canale predefinito (o un canale specifico).

    // Ottieni i dati reali del server
    let user_data = User {
        id: new_member.user.id.get() as i64,
        joined_at: new_member.joined_at.unwrap().to_string(),
        username: new_member.user.name.clone(),
        discord_id: new_member.guild_id.get() as i64,
        global_name: new_member.user.global_name.unwrap_or_default(),
        avatar: new_member.user.avatar.clone().map(|image_hash| image_hash.to_string()),
    };

    // Construct the URL
    // let url = format!("https://localhost:8000/api/v1/user/@me");
    // let body = Some(json!({
    //     "id": user_data.id,
    //     "joined_at": user_data.joined_at,
    //     "username": user_data.username,
    //     "global_name": user_data.global_name,
    //     "discord_id": user_data.discord_id,
    //     "avatar": user_data.avatar
    // }));

    // let result = fetch_data(FetchType::PUT, &url, body).await;

    let err = "no err";
    let response_text = "no response text";
    let result: Result<String, String> = Ok("test".to_owned()); // Changed to Some(true) to match Option type

    match result {
        Ok(response) => {
            println!("API call successful. Response: {}", response_text);
        },
        Err(err) => {
            println!("API call failed: {:?}", err);
        }
    }
}
