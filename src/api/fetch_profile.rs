use reqwasm::http::Request;
use serde::Deserialize;
use web_sys::RequestCredentials;

use crate::api::models::{ApiError, ApiErrorPayload};
use crate::constants::ENV_CONFIG;
use crate::contexts::auth_context::Profile;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ProfileResDto {
    pub id: i32,
    pub login: String,
}

impl Into<Profile> for ProfileResDto {
    fn into(self) -> Profile {
        Profile {
            id: self.id,
            login: self.login,
        }
    }
}

pub async fn fetch_profile() -> Result<ProfileResDto, ApiError> {
    let result = Request::get((ENV_CONFIG.clone_api_base_url() + "/users/profile").as_str())
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

    let result = response.json::<ProfileResDto>().await;

    if let Err(error) = result {
        return Err(error.into());
    }

    let profile = result.unwrap();

    Ok(profile)
}
