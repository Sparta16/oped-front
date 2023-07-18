use reqwasm::http::Request;
use serde::Deserialize;
use web_sys::RequestCredentials;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::UsersTable;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct UserResDto {
    pub id: i32,
    pub login: String,
}

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    let users = use_state(|| vec![]);

    let cloned_users = users.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let result = Request::get("http://localhost:25565/api/v1/users")
                    .credentials(RequestCredentials::Include)
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<UserResDto>>()
                    .await
                    .unwrap();

                cloned_users.set(result);
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
