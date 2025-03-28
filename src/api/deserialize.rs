use crate::api::errors::FetchError;
use crate::models::discord::Discord;    

// Funzione per deserializzare il JSON
pub fn deserialize_json(response: &str) -> Result<Discord, FetchError> {
    serde_json::from_str(response).map_err(FetchError::SerdeError)
}