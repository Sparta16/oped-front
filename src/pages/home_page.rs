use yew::prelude::*;

use crate::context::AuthContext;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let auth_context = use_context::<AuthContext>().unwrap();

    html! {
        <main class="grid place-items-center gap-2 pt-4">
            <h1>{"Главная страница"}</h1>
            if let Some(profile) = auth_context.profile.clone() {
                <p>{profile.login}</p>
            }
        </main>
    }
}
