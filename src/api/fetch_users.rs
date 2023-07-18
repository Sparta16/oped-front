use crate::constants::ENV_CONFIG;
use reqwasm::http::Request;
use reqwasm::Error;
use serde::Deserialize;
use web_sys::RequestCredentials;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct UserResDto {
    pub id: i32,
    pub login: String,
}

pub async fn fetch_users() -> Result<Vec<UserResDto>, Error> {
    let result = Request::get((ENV_CONFIG.clone_api_base_url() + "/users").as_str())
        .header("content-type", "application/json")
        .credentials(RequestCredentials::Include)
        .send()
        .await;

    if let Err(error) = result {
        return Err(error);
    }

    let response = result.unwrap();

    let result = response.json::<Vec<UserResDto>>().await;

    if let Err(error) = result {
        return Err(error);
    }

    let users = result.unwrap();

    Ok(users)
}
