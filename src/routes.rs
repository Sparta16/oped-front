use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Header;
use crate::hocs::WithAuth;
use crate::pages::{HomePage, LoginPage, NotFoundPage, RegistrationPage, UsersPage};

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <Switch<AppRoute> render={switch_app_route} />
    }
}

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
    #[at("/users")]
    Users,
    #[at("/registration")]
    Registration,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_main_route(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<HomePage />},
        MainRoute::Users => html! {
            <WithAuth denied_children={html! {<Redirect<AppRoute> to={AppRoute::NotFound} />}}>
                <UsersPage />
            </WithAuth>
        },
        MainRoute::Registration => html! {<RegistrationPage />},
        MainRoute::Login => html! {<LoginPage />},
        MainRoute::NotFound => html! {<Redirect<AppRoute> to={AppRoute::NotFound} />},
    }
}
