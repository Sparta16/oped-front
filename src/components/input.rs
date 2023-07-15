use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(String::from(""))]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub on_input: Callback<String>,
    #[prop_or(String::from(""))]
    pub placeholder: String,
    #[prop_or(String::from("text"))]
    pub input_type: String,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let handle_input = {
        let on_input = props.on_input.clone();

        move |event: InputEvent| {
            event.prevent_default();

            let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();

            on_input.emit(input.value());
        }
    };

    html! {
        <input
            placeholder={props.placeholder.clone()}
            oninput={handle_input}
            type={props.input_type.clone()}
            class={"bg-pink-200 rounded-lg px-2 outline-gray-300 ".to_string() + props.class.clone().as_str()}
        />
    }
}
