use reqwasm::http::Request;
use serde::Deserialize;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Deserialize, Debug)]
struct UserResDto {
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
        <main>
            <h1>{"Список пользователей"}</h1>
            <ul>
                {
                    (*users).iter().map(|user| {
                        html! {
                            <li key={user.id}>{user.login.clone()}</li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </main>
    }
}
