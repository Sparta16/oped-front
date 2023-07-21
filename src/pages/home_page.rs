use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <main class="grid place-items-center gap-2 pt-4 h-full auto-rows-minmax">
            <h1>{"Главная страница"}</h1>
        </main>
    }
}
