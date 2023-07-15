use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::MainRoute;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="py-1 px-4 border-b border-grey-900">
            <div class="grid grid-flow-col gap-2 auto-cols-minmax">
                <Link<MainRoute> to={MainRoute::Home}>{"Главная"}</Link<MainRoute>>
                <Link<MainRoute> to={MainRoute::Users}>{"Пользователи"}</Link<MainRoute>>
                <Link<MainRoute> to={MainRoute::Registration}>{"Регистрация"}</Link<MainRoute>>
                <Link<MainRoute> to={MainRoute::Login}>{"Вход"}</Link<MainRoute>>
            </div>
        </header>
    }
}
