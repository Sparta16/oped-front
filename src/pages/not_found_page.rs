use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <main class="grid place-items-center gap-2 pt-4">
            <h1>{"Такой страницы не существует"}</h1>
        </main>
    }
}
