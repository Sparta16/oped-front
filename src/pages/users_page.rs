use yew::platform::spawn_local;
use yew::prelude::*;

use crate::api::fetch_users;
use crate::components::UsersTable;

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    let users = use_state(Vec::new);

    let cloned_users = users.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let result = fetch_users().await;

                if let Ok(users) = result {
                    cloned_users.set(users);
                }
            });
        },
        (),
    );

    html! {
        <main class="grid place-items-center gap-2 pt-4 auto-rows-minmax">
            <h1>{"Список пользователей"}</h1>
            <UsersTable users={(*users).clone()} />
        </main>
    }
}
