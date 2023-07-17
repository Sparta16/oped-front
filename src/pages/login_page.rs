use gloo::dialogs::alert;
use reqwasm::http::Request;
use web_sys::RequestCredentials;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::login_form::LoginFormValues;
use crate::components::LoginForm;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let handle_submit = {
        |payload: LoginFormValues| {
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
