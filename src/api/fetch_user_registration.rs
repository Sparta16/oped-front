use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::RequestCredentials;

use crate::api::models::{ApiError, ApiErrorPayload, ErrorDto};
use crate::constants::ENV_CONFIG;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserRegistrationReqDto {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct UserRegistrationResDto {}

pub async fn fetch_user_registration(
    payload: UserRegistrationReqDto,
) -> Result<UserRegistrationResDto, ApiError> {
    let result = Request::post((ENV_CONFIG.clone_api_base_url() + "/users/registration").as_str())
        .header("content-type", "application/json")
        .credentials(RequestCredentials::Include)
        .body(serde_json::to_string(&payload).unwrap())
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
        }

        let dto = result.unwrap();

        Err(ApiError::Payload(ApiErrorPayload {
            code: response.status(),
            message: dto.message,
        }))
    } else {
        let result = response.json::<UserRegistrationResDto>().await;

        if let Err(error) = result {
            return Err(error.into());
        }

        let dto = result.unwrap();

        Ok(dto)
    }
}
