use gloo::dialogs::alert;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::{fetch_user_registration, models::ApiError};
use crate::components::registration_form::RegistrationFormValues;
use crate::components::RegistrationForm;
use crate::routes::MainRoute;

#[function_component(RegistrationPage)]
pub fn registration_page() -> Html {
    let navigator = use_navigator().unwrap();

    let handle_submit = {
        move |payload: RegistrationFormValues| {
            let navigator = navigator.clone();

            spawn_local(async move {
                let result = fetch_user_registration(payload.into()).await;

                match result {
                    Ok(_) => {
                        alert("OK");
                        navigator.push(&MainRoute::Login);
                    }
                    Err(ApiError::Payload(payload)) => {
                        alert(payload.message.as_str());
                    }
                    Err(ApiError::Fetch(message)) => {
                        alert(message.as_str());
                    }
                }
            });
        }
    };

    html! {
        <main class="grid place-items-center gap-2 pt-4 auto-rows-minmax">
            <h1>{"Регистрация"}</h1>
            <RegistrationForm on_submit={handle_submit} />
            <Link<MainRoute> classes="text-xs opacity-75 hover:opacity-100" to={MainRoute::Login}>{"Уже есть аккаунт? Авторизироваться"}</Link<MainRoute>>
        </main>
    }
}
