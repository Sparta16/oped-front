use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(String::from(""))]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub on_click: Callback<MouseEvent>,
    #[prop_or(String::from("Отправить"))]
    pub text: String,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let handle_click = {
        let on_click = props.on_click.clone();

        move |event: MouseEvent| on_click.emit(event)
    };

    html! {
        <button
            onclick={handle_click}
            class={"bg-pink-200 rounded-lg px-4 ".to_string() + props.class.clone().as_str()}
        >
            {props.text.clone()}
        </button>
    }
}
