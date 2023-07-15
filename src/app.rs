use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch_app_route, AppRoute};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch_app_route} />
        </BrowserRouter>
    }
}
