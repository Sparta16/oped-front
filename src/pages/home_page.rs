use yew::prelude::*;

use crate::contexts::use_auth_context;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let auth_context = use_auth_context();

    html! {
        <main class="grid place-items-center gap-2 pt-4 h-full auto-rows-minmax">
            <h1>{"Главная страница"}</h1>
            if let Some(profile) = &auth_context.profile {
                <p>{"Привет, "}{profile.login.clone()}{"!"}</p>
            }
        </main>
    }
}
