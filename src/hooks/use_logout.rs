use reqwasm::http::Request;
use web_sys::RequestCredentials;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::contexts::{use_auth_context, AuthContextAction};
use crate::routes::MainRoute;

#[hook]
pub fn use_logout() -> Callback<()> {
    let auth_context = use_auth_context();
    let navigator = use_navigator().unwrap();

    Callback::from(move |_| {
        let auth_context = auth_context.clone();
        let navigator = navigator.clone();
        spawn_local(async move {
            Request::post("http://localhost:25565/api/v1/users/logout")
                .header("content-type", "application/json")
                .credentials(RequestCredentials::Include)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            navigator.push(&MainRoute::Home);
            auth_context.dispatch(AuthContextAction::SetProfile(None));
        });
    })
}
