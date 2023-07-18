use gloo::dialogs::alert;
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
}

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html {
    let form_state = use_state(FormState::default);

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

            if form_state.login.len() < 3 || form_state.login.len() > 30 {
                return alert("Длина логина: 3-30");
            }

            if form_state.password.len() < 3 || form_state.password.len() > 30 {
                return alert("Длина пароля: 3-30");
            }

            on_submit.emit(LoginFormValues::new(
                form_state.login.clone(),
                form_state.password.clone(),
            ))
        }
    };

    html! {
        <form onsubmit={handle_submit} class="p-4 grid gap-2 place-items-center w-60 border border-grey-900 rounded-lg">
            <Input class="w-full" placeholder="Логин" on_input={handle_login_input} />
            <Input class="w-full" placeholder="Пароль" input_type="password" on_input={handle_password_input} />
            <Button text="Авторизироваться" />
        </form>
    }
}
