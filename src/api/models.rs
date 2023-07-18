use reqwasm::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorDto {
    pub message: String,
}

pub enum ApiError {
    Fetch(String),
    Payload(ApiErrorPayload),
}

pub struct ApiErrorPayload {
    pub code: u16,
    pub message: String,
}

impl Into<ApiError> for Error {
    fn into(self) -> ApiError {
        ApiError::Fetch(self.to_string())
    }
}
