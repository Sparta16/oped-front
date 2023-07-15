use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <main>{"Такой страницы не существует"}</main>
    }
}
