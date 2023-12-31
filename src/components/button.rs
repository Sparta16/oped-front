use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub on_click: Callback<()>,
    #[prop_or(String::from("Отправить"))]
    pub text: String,
    #[prop_or_default]
    pub is_loading: bool,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let handle_click = {
        let on_click = props.on_click.clone();

        move |_| on_click.emit(())
    };

    html! {
        <button
            disabled={props.is_loading}
            onclick={handle_click}
            class={"bg-sky-100 rounded-lg px-4 outline-gray-300 ".to_owned() + props.class.clone().as_str()}
        >
            {props.text.clone()}
        </button>
    }
}
