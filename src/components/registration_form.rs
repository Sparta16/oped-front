use ::yew::prelude::*;
use gloo::dialogs::alert;

use crate::components::{Button, Input};

pub struct RegistrationFormValues {
    login: String,
    password: String,
}

impl RegistrationFormValues {
    pub fn new(login: String, password: String) -> Self {
        Self { login, password }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(Callback::noop())]
    pub on_submit: Callback<RegistrationFormValues>,
}

#[function_component(RegistrationForm)]
pub fn registration_form(props: &Props) -> Html {
    let login = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let repeated_password = use_state(|| "".to_string());

    let handle_login_input = {
        let login = login.clone();
        move |value: String| login.set(value)
    };

    let handle_password_input = {
        let password = password.clone();
        move |value: String| password.set(value)
    };

    let handle_repeated_password_input = {
        let repeated_password = repeated_password.clone();
        move |value: String| repeated_password.set(value)
    };

    let handle_submit = {
        let on_submit = props.on_submit.clone();
        let login = login.clone();
        let password = password.clone();
        let repeated_password = repeated_password.clone();
        move |event: SubmitEvent| {
            event.prevent_default();

            if (*login).len() < 3 || (*login).len() > 30 {
                return alert("Длина логина: 3-30");
            }

            if (*password).len() < 3
                || (*password).len() > 30
                || (*repeated_password).len() < 3
                || (*repeated_password).len() > 30
            {
                return alert("Длина пароля: 3-30");
            }

            if *password != *repeated_password {
                return alert("Пароли не совпадают");
            }

            on_submit.emit(RegistrationFormValues::new(
                (*login).clone(),
                (*password).clone(),
            ))
        }
    };

    html! {
        <form onsubmit={handle_submit} class="p-4 grid gap-2 place-items-center w-60 border border-grey-900 rounded-lg">
            <Input class="w-full" placeholder="Логин" on_input={handle_login_input} />
            <Input class="w-full" placeholder="Пароль" input_type="password" on_input={handle_password_input} />
            <Input class="w-full" placeholder="Повторите пароль" input_type="password" on_input={handle_repeated_password_input} />
            <Button text="Зарегистрироваться" />
        </form>
    }
}
