use gloo::dialogs::alert;
use reqwasm::http::Request;
use web_sys::RequestCredentials;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::login_form::LoginFormValues;
use crate::components::LoginForm;
use crate::context::{AuthContext, AuthContextAction};

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let auth_context = use_context::<AuthContext>().unwrap();

    let handle_submit = {
        move |payload: LoginFormValues| {
            let auth_context = auth_context.clone();
            spawn_local(async move {
                let result = Request::post("http://localhost:25565/api/v1/users/login")
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .credentials(RequestCredentials::Include)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                alert(result.as_str());

                auth_context.dispatch(AuthContextAction::RequestFetch);
            });
        }
    };

    html! {
        <main class="grid place-items-center gap-2 pt-4">
            <h1>{"Авторизация"}</h1>
            <LoginForm on_submit={handle_submit} />
        </main>
    }
}
