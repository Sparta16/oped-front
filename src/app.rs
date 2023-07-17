use yew::prelude::*;
use yew_router::prelude::*;

use crate::context::AuthProvider;
use crate::routes::{switch_app_route, AppRoute};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AuthProvider>
            <BrowserRouter>
                <Switch<AppRoute> render={switch_app_route} />
            </BrowserRouter>
        </AuthProvider>
    }
}
