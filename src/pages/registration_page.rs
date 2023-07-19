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

    let is_loading_state = use_state(bool::default);
    let error_state = use_state(|| None);

    let handle_submit = {
        let is_loading_state = is_loading_state.clone();
        let error_state = error_state.clone();

        move |payload: RegistrationFormValues| {
            let navigator = navigator.clone();

            is_loading_state.set(true);
            error_state.set(None);

            let is_loading_state = is_loading_state.clone();
            let error_state = error_state.clone();

            spawn_local(async move {
                let result = fetch_user_registration(payload.into()).await;

                match result {
                    Ok(_) => {
                        navigator.push(&MainRoute::Login);
                    }
                    Err(ApiError::Payload(payload)) => {
                        error_state.set(Some(payload.message));
                    }
                    Err(ApiError::Fetch(message)) => {
                        error_state.set(Some(message));
                    }
                };

                is_loading_state.set(false);
            });
        }
    };

    html! {
        <main class="grid place-items-center gap-2 pt-4 auto-rows-minmax">
            <h1>{"Регистрация"}</h1>
            <RegistrationForm error={(*error_state).clone()} is_loading={*is_loading_state} on_submit={handle_submit} />
            <Link<MainRoute> classes="text-xs opacity-75 hover:opacity-100" to={MainRoute::Login}>{"Уже есть аккаунт? Авторизироваться"}</Link<MainRoute>>
        </main>
    }
}
