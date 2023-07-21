use yew::prelude::*;
use yew_router::prelude::*;

use crate::contexts::use_auth_context;
use crate::hooks::use_logout;
use crate::routes::{MainRoute, UsersRoute};

#[function_component(Header)]
pub fn header() -> Html {
    let auth_context = use_auth_context();
    let logout = use_logout();

    let handle_logout_click = {
        move |_| {
            logout.emit(());
        }
    };

    html! {
        <header class="py-1 px-4 border-b border-grey-900">
            <div class="grid grid-flow-col gap-2 auto-cols-minmax">
                <Link<MainRoute> to={MainRoute::Home}>{"Главная"}</Link<MainRoute>>
                <Link<MainRoute> to={MainRoute::UsersRoot}>{"Пользователи"}</Link<MainRoute>>
                if let Some(profile) = &auth_context.profile {
                    <Link<UsersRoute> to={UsersRoute::User { login: profile.login.clone() }}>{"Профиль"}</Link<UsersRoute>>
                    <button onclick={handle_logout_click}>{"Выход"}</button>
                } else {
                    <Link<MainRoute> to={MainRoute::Login}>{"Вход"}</Link<MainRoute>>
                }
            </div>
        </header>
    }
}
