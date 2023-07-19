use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::{fetch_user_login, models::ApiError};
use crate::components::login_form::LoginFormValues;
use crate::components::LoginForm;
use crate::contexts::{use_auth_context, AuthContextAction};
use crate::routes::MainRoute;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let auth_context = use_auth_context();
    let navigator = use_navigator().unwrap();

    let is_loading_state = use_state(bool::default);
    let error_state = use_state(|| None);

    let handle_submit = {
        let is_loading_state = is_loading_state.clone();
        let error_state = error_state.clone();

        move |payload: LoginFormValues| {
            let auth_context = auth_context.clone();
            let navigator = navigator.clone();

            is_loading_state.set(true);
            error_state.set(None);

            let is_loading_state = is_loading_state.clone();
            let error_state = error_state.clone();

            spawn_local(async move {
                let result = fetch_user_login(payload.into()).await;

                match result {
                    Ok(_) => {
                        auth_context.dispatch(AuthContextAction::RequestFetch);
                        navigator.push(&MainRoute::Home);
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
            <h1>{"Авторизация"}</h1>
            <LoginForm error={(*error_state).clone()} is_loading={*is_loading_state} on_submit={handle_submit} />
            <Link<MainRoute> classes="text-xs opacity-75 hover:opacity-100" to={MainRoute::Registration}>{"Нет аккаунта? Зарегистрироваться"}</Link<MainRoute>>
        </main>
    }
}
