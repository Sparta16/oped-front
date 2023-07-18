use yew::prelude::*;
use yew_router::prelude::*;

use crate::contexts::use_auth_context;
use crate::hooks::use_logout;
use crate::routes::MainRoute;

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
                if let Some(_) = auth_context.profile {
                    <>
                        <Link<MainRoute> to={MainRoute::Users}>{"Пользователи"}</Link<MainRoute>>
                        <button onclick={handle_logout_click}>{"Выход"}</button>
                    </>
                } else {
                    <Link<MainRoute> to={MainRoute::Login}>{"Вход"}</Link<MainRoute>>
                }
            </div>
        </header>
    }
}
