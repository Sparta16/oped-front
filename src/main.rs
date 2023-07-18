mod app;
mod components;
mod contexts;
mod hocs;
mod hooks;
mod pages;
mod routes;

use yew::Renderer;

use crate::app::App;

fn main() {
    Renderer::<App>::new().render();
}
