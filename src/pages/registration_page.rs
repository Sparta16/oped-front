use gloo::dialogs::alert;
use reqwasm::http::Request;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::{RegistrationForm, RegistrationFormValues};

#[function_component(RegistrationPage)]
pub fn registration_page() -> Html {
    let handle_submit = {
        |payload: RegistrationFormValues| {
            spawn_local(async move {
                let result = Request::post("http://localhost:25565/api/v1/users/registration")
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
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
            <h1>{"Регистрация"}</h1>
            <RegistrationForm on_submit={handle_submit} />
        </main>
    }
}
