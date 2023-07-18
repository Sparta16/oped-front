use reqwasm::http::Request;
use serde::Deserialize;
use web_sys::RequestCredentials;

use crate::api::models::{ApiError, ApiErrorPayload};
use crate::constants::ENV_CONFIG;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct UserResDto {
    pub id: i32,
    pub login: String,
}

pub async fn fetch_users() -> Result<Vec<UserResDto>, ApiError> {
    let result = Request::get((ENV_CONFIG.clone_api_base_url() + "/users").as_str())
        .header("content-type", "application/json")
        .credentials(RequestCredentials::Include)
        .send()
        .await;

    if let Err(error) = result {
        return Err(error.into());
    }

    let response = result.unwrap();

    if response.status() >= 400 {
        return Err(ApiError::Payload(ApiErrorPayload {
            code: response.status(),
            message: response.text().await.unwrap_or_default(),
        }));
    }

    let result = response.json::<Vec<UserResDto>>().await;

    if let Err(error) = result {
        return Err(error.into());
    }

    let users = result.unwrap();

    Ok(users)
}
