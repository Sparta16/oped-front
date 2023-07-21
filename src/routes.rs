use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Header;
use crate::hocs::{WithAuth, WithoutAuth};
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

fn switch_app_route(route: AppRoute) -> Html {
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
    UsersRoot,
    #[at("/users/*")]
    Users,
    #[at("/registration")]
    Registration,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch_main_route(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<HomePage />},
        MainRoute::UsersRoot | MainRoute::Users => html! {
            <Switch<UsersRoute> render={switch_users_route}/>
        },
        MainRoute::Registration => html! {
            <WithoutAuth denied_children={html! {<Redirect<AppRoute> to={AppRoute::NotFound} />}}>
                <RegistrationPage />
            </WithoutAuth>
        },
        MainRoute::Login => html! {
            <WithoutAuth denied_children={html! {<Redirect<AppRoute> to={AppRoute::NotFound} />}}>
                <LoginPage />
            </WithoutAuth>
        },
        MainRoute::NotFound => html! {<Redirect<AppRoute> to={AppRoute::NotFound} />},
    }
}
