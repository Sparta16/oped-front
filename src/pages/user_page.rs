use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::fetch_user;
use crate::routes::AppRoute;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub login: String,
}

#[function_component(UserPage)]
pub fn user_page(props: &Props) -> Html {
    let user_state = use_state(|| None);
    let navigator = use_navigator().unwrap();

    let login = props.login.clone();
    let cloned_login = login.clone();
    let cloned_user_state = user_state.clone();
    use_effect_with_deps(
        move |_| {
            let cloned_login = cloned_login.clone();
            let cloned_user_state = cloned_user_state.clone();

            spawn_local(async move {
                let result = fetch_user(&cloned_login).await;

                match result {
                    Ok(user) => {
                        cloned_user_state.set(Some(user));
                    }
                    Err(_) => {
                        cloned_user_state.set(None);

                        navigator.push(&AppRoute::NotFound);
                    }
                }
            });
        },
        login,
    );

    html! {
        <main class="grid place-items-center gap-2 pt-4 auto-rows-minmax">
            if let Some(user) = (*user_state).clone() {
                <p>{&user.login}</p>
            }
        </main>
    }
}
