mod app;

use yew::Renderer;

use crate::app::App;

fn main() {
    Renderer::<App>::new().render();
}
