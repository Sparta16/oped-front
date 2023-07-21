use reqwasm::http::Request;
use serde::Deserialize;
use web_sys::RequestCredentials;

use crate::api::models::{ApiError, ApiErrorPayload, ErrorDto};
use crate::constants::ENV_CONFIG;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct UserResDto {
    pub id: i32,
    pub login: String,
}

pub async fn fetch_user(login: &str) -> Result<UserResDto, ApiError> {
    let result = Request::get(&(ENV_CONFIG.clone_api_base_url() + "/users/" + login))
        .header("content-type", "application/json")
        .credentials(RequestCredentials::Include)
        .send()
        .await;

    if let Err(error) = result {
        return Err(error.into());
    }

    let response = result.unwrap();

    if response.status() >= 400 {
        let result = response.json::<ErrorDto>().await;

        if let Err(error) = result {
            return Err(error.into());
        };

        let dto = result.unwrap();

        Err(ApiError::Payload(ApiErrorPayload {
            code: response.status(),
            message: dto.message,
        }))
    } else {
        let result = response.json::<UserResDto>().await;

        if let Err(error) = result {
            return Err(error.into());
        }

        let dto = result.unwrap();

        Ok(dto)
    }
}
