mod app;
mod components;
mod context;
mod pages;
mod routes;

use yew::Renderer;

use crate::app::App;

fn main() {
    Renderer::<App>::new().render();
}
