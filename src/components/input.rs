use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub on_input: Callback<String>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or(String::from("text"))]
    pub input_type: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or(u16::MIN)]
    pub min_length: u16,
    #[prop_or(u16::MAX)]
    pub max_length: u16,
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
            required={props.required}
            minLength={props.min_length.to_string()}
            maxLength={props.max_length.to_string()}
            class={"bg-sky-100 rounded-lg px-2 outline-gray-300 ".to_owned() + props.class.clone().as_str()}
        />
    }
}
