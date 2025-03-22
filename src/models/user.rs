use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub discord_id: i64,
    pub global_name: String,
    pub joined_at: String,
    pub avatar: Option<String>,
}