use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{HomePage, LoginPage, NotFoundPage, RegistrationPage};

use crate::components::Header;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    MainRoot,
    #[at("/*")]
    Main,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_app_route(route: AppRoute) -> Html {
    match route {
        AppRoute::MainRoot | AppRoute::Main => html! {
            <>
                <Header />
                <Switch<MainRoute> render={switch_main_route}/>
            </>
        },
        AppRoute::NotFound => html! {<NotFoundPage />},
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/registration")]
    Registration,
    #[at("/login")]
    Login,
}

pub fn switch_main_route(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<HomePage />},
        MainRoute::Registration => html! {<RegistrationPage />},
        MainRoute::Login => html! {<LoginPage />},
    }
}
