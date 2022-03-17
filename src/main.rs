use yew::prelude::*;

fn main() {
    yew::start_app::<Root>();
}

#[function_component(Root)]
fn root() -> Html {
    let onclick: Callback<MouseEvent> = Callback::from(|mouse_event: MouseEvent| {
        web_sys::console::log_1(&mouse_event.into())
    });


    html! {
        <div>
            <button {onclick}>{"Sign in"}</button>
        </div>
    }
}