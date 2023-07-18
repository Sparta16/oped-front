use serde::Deserialize;
use std::rc::Rc;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::api::fetch_profile;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Profile {
    pub id: i32,
    pub login: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthContextState {
    pub profile: Option<Profile>,
    pub should_fetch: bool,
}

impl Default for AuthContextState {
    fn default() -> Self {
        Self {
            profile: None,
            should_fetch: true,
        }
    }
}

pub enum AuthContextAction {
    SetProfile(Option<Profile>),
    RequestFetch,
}

pub type AuthContext = UseReducerHandle<AuthContextState>;

impl Reducible for AuthContextState {
    type Action = AuthContextAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthContextAction::SetProfile(profile) => Self {
                profile,
                should_fetch: false,
            },
            AuthContextAction::RequestFetch => Self {
                should_fetch: true,
                ..(*self).clone()
            },
        }
        .into()
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &Props) -> Html {
    let auth_context = use_reducer(AuthContextState::default);

    let cloned_auth_context = auth_context.clone();
    use_effect_with_deps(
        |_| {
            if (*cloned_auth_context).should_fetch {
                spawn_local(async move {
                    let result = fetch_profile().await;

                    match result {
                        Ok(profile) => {
                            cloned_auth_context
                                .dispatch(AuthContextAction::SetProfile(Some(profile.into())));
                        }
                        Err(_) => {
                            cloned_auth_context.dispatch(AuthContextAction::SetProfile(None));
                        }
                    }
                });
            }
        },
        (*auth_context).should_fetch,
    );

    html! {
        <ContextProvider<UseReducerHandle<AuthContextState>> context={auth_context.clone()}>
            {props.children.clone()}
        </ContextProvider<UseReducerHandle<AuthContextState>>>
    }
}

#[hook]
pub fn use_auth_context() -> AuthContext {
    use_context::<AuthContext>().expect("AuthContext must be provided by AuthProvider")
}
