use std::fmt;

// Definisci un tipo di errore personalizzato
#[derive(Debug)]
pub enum FetchError {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
    NoData,
    UnexpectedStatus,
}

// Implementa Display per FetchError
impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchError::ReqwestError(e) => write!(f, "API call error: {}", e),
            FetchError::SerdeError(e) => write!(f, "JSON deserialization error: {}", e),
            FetchError::NoData => write!(f, "No data found"),
            FetchError::UnexpectedStatus => write!(f, "Unexpected response status"),
        }
    }
}

// Implementa From per convertire reqwest::Error e serde_json::Error in FetchError
impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> FetchError {
        FetchError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::SerdeError(err)
    }
}