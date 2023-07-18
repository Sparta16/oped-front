mod api;
mod app;
mod components;
mod constants;
mod contexts;
mod hocs;
mod hooks;
mod models;
mod pages;
mod routes;

use yew::Renderer;

use crate::app::App;

fn main() {
    Renderer::<App>::new().render();
}
