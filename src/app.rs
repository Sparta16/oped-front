use yew::prelude::*;
use yew_router::prelude::*;

use crate::contexts::AuthProvider;
use crate::routes::Router;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AuthProvider>
            <BrowserRouter>
                <Router />
            </BrowserRouter>
        </AuthProvider>
    }
}
