use crate::contexts::use_auth_context;
use yew::prelude::*;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let auth_context = use_auth_context();

    let profile = auth_context.profile.clone().unwrap();

    html! {
        <main class="grid place-items-center gap-2 pt-4 auto-rows-minmax">
            <p>{"Привет, "}{&profile.login}{"!"}</p>
        </main>
    }
}
