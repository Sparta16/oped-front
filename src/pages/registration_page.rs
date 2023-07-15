use gloo::dialogs::alert;
use yew::prelude::*;

use crate::components::{RegistrationForm, RegistrationFormValues};

#[function_component(RegistrationPage)]
pub fn registration_page() -> Html {
    let handle_submit = {
        |payload: RegistrationFormValues| {
            alert("Зарегались");
        }
    };

    html! {
        <main class="grid place-items-center gap-2 pt-4">
            <h1>{"Регистрация"}</h1>
            <RegistrationForm on_submit={handle_submit} />
        </main>
    }
}
