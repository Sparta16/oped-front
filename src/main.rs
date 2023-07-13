use lazy_static::lazy_static;
use yew::prelude::*;

lazy_static! {
    static ref OMG: Vec<i32> = {
        let mut omg = vec![];

        for i in (6..=1000).step_by(7) {
            omg.push(i);
        }

        omg.reverse();

        omg
    };
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 1000);

    let counter_minus = counter.clone();
    let handle_minus_click = Callback::from(move |_| {
        counter_minus.set(*counter_minus - 7);
    });

    let counter_plus = counter.clone();
    let handle_plus_click = Callback::from(move |_| {
        counter_plus.set(*counter_plus + 7);
    });

    html! {
        <div>
            <button onclick={handle_plus_click}>{ "+7" }</button>
            <button onclick={handle_minus_click}>{ "-7" }</button>
            <p>{ *counter }</p>
            <ul>
                {
                    OMG.iter().filter(|number| **number <= *counter).map(|number| {
                        html! {
                            <li key={*number}>{ format!("{number}")}</li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
