use yew::prelude::*;

use crate::contexts::use_auth_context;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub denied_children: Children,
    pub login: String,
}

#[function_component(WithProfile)]
pub fn with_profile(props: &Props) -> Html {
    let auth_context = use_auth_context();

    match &auth_context.profile {
        Some(profile) if profile.login == props.login => html! {<>{props.children.clone()}</>},
        _ => html! {<>{props.denied_children.clone()}</>},
    }
}
