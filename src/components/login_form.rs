use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::api::fetch_user_login::UserLoginReqDto;
use crate::components::{Button, Input};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginFormValues {
    login: String,
    password: String,
}

impl LoginFormValues {
    pub fn new(login: String, password: String) -> Self {
        Self { login, password }
    }
}

impl Into<UserLoginReqDto> for LoginFormValues {
    fn into(self) -> UserLoginReqDto {
        UserLoginReqDto {
            login: self.login,
            password: self.password,
        }
    }
}

#[derive(Default, Clone)]
struct FormState {
    pub login: String,
    pub password: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub on_submit: Callback<LoginFormValues>,
    pub error: Option<String>,
    pub is_loading: bool,
}

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html {
    let form_state = use_state(FormState::default);
    let error_state = use_state(|| None);

    let error = props.error.clone();
    let cloned_error = error.clone();
    let cloned_error_state = error_state.clone();
    use_effect_with_deps(
        move |_| {
            if cloned_error.is_some() {
                cloned_error_state.set(cloned_error);
            }
        },
        error,
    );

    let handle_login_input = {
        let form_state = form_state.clone();
        move |value: String| {
            form_state.set(FormState {
                login: value,
                ..(*form_state).clone()
            });
        }
    };

    let handle_password_input = {
        let form_state = form_state.clone();
        move |value: String| {
            form_state.set(FormState {
                password: value,
                ..(*form_state).clone()
            });
        }
    };

    let handle_submit = {
        let on_submit = props.on_submit.clone();
        let form_state = form_state.clone();
        move |event: SubmitEvent| {
            event.prevent_default();

            let form_state = (*form_state).clone();

            on_submit.emit(LoginFormValues::new(
                form_state.login.clone(),
                form_state.password.clone(),
            ))
        }
    };

    html! {
        <form onsubmit={handle_submit} class="p-4 grid gap-2 place-items-center w-60 border border-grey-900 rounded-lg">
            <Input class="w-full" placeholder="Логин" required={true} min_length={3} max_length={30} on_input={handle_login_input} />
            <Input class="w-full" placeholder="Пароль" required={true} min_length={3} max_length={30} input_type="password" on_input={handle_password_input} />
            <Button is_loading={props.is_loading} text="Авторизироваться" />
            if let Some(error_message) = (*error_state).clone() {
                <p class="text-xs text-red-600">{error_message}</p>
            }
        </form>
    }
}
