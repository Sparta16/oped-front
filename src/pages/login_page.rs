use gloo::dialogs::alert;
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

    let handle_submit = {
        move |payload: LoginFormValues| {
            let auth_context = auth_context.clone();
            let navigator = navigator.clone();

            spawn_local(async move {
                let result = fetch_user_login(payload.into()).await;

                match result {
                    Ok(_) => {
                        alert("OK");
                        auth_context.dispatch(AuthContextAction::RequestFetch);
                        navigator.push(&MainRoute::Home);
                    }
                    Err(ApiError::Payload(payload)) => {
                        alert(payload.message.as_str());
                    }
                    Err(ApiError::Fetch(message)) => alert(message.as_str()),
                }
            });
        }
    };

    html! {
        <main class="grid place-items-center gap-2 pt-4 auto-rows-minmax">
            <h1>{"Авторизация"}</h1>
            <LoginForm on_submit={handle_submit} />
            <Link<MainRoute> classes="text-xs opacity-75 hover:opacity-100" to={MainRoute::Registration}>{"Нет аккаунта? Зарегистрироваться"}</Link<MainRoute>>
        </main>
    }
}
