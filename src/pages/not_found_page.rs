use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::MainRoute;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <main class="grid place-items-center gap-2 pt-4 auto-rows-minmax">
            <h1>{"Такой страницы не существует"}</h1>
            <Link<MainRoute> to={MainRoute::Home} classes="bg-sky-100 rounded-lg px-4">{"На главную"}</Link<MainRoute>>
        </main>
    }
}
