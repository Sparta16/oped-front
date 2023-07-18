use yew::prelude::*;

use crate::contexts::use_auth_context;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub denied_children: Children,
}

#[function_component(WithAuth)]
pub fn with_auth(props: &Props) -> Html {
    let auth_context = use_auth_context();

    html! {
        if let Some(_) = &auth_context.profile {
            {props.children.clone()}
        } else {
            {props.denied_children.clone()}
        }
    }
}
