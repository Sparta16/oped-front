use yew::prelude::*;

use crate::api::fetch_users::UserResDto;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub users: Vec<UserResDto>,
}

#[function_component(UsersTable)]
pub fn users_table(props: &Props) -> Html {
    html! {
        <table class="border border-grey-900 w-fit">
            <thead class="bg-sky-200">
                <tr>
                    <th class="border border-grey-900 px-2">{"Id"}</th>
                    <th class="border border-grey-900 px-2">{"Login"}</th>
                </tr>
            </thead>
            <tbody>
                {
                    (*props.users).iter().map(|user| {
                        html! {
                            <tr key={user.id}>
                                <td class="border border-grey-900 px-2">{user.id}</td>
                                <td class="border border-grey-900 px-2">{user.login.clone()}</td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
