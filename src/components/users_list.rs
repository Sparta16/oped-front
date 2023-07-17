use yew::prelude::*;

use crate::pages::users_page::UserResDto;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub users: Vec<UserResDto>,
}

#[function_component(UsersList)]
pub fn users_list(props: &Props) -> Html {
    html! {
        <ul>
            {
                (*props.users).iter().map(|user| {
                    html! {
                        <li key={user.id}>{user.login.clone()}</li>
                    }
                }).collect::<Html>()
            }
        </ul>
    }
}
