use crate::{
    context::{WordAction, WordsContextInfo, WordsContextType},
    row::Row,
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::KeyboardEvent;
use yew::{
    function_component, html, use_context, use_effect_with_deps, Html,
};
#[function_component(MainApp)]
pub fn component() -> Html {
    let word_info = use_context::<WordsContextType>().unwrap();

    {
        let word_info = word_info.clone();
        use_effect_with_deps(
            move |_| {
                let window = web_sys::window().unwrap();
                let listener = Closure::<dyn Fn(KeyboardEvent)>::wrap(
                    Box::new(move |ev| {
                        let a = ev.key();
                        if a == "Backspace" {
                            word_info.dispatch(WordAction::Back);
                        } else if a.len() == 1 {
                            let c = a.chars().collect::<Vec<char>>()[0];
                            word_info.dispatch(WordAction::Push(c));
                        }
                    }),
                );
                window
                    .add_event_listener_with_callback(
                        "keydown",
                        listener.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                move || {
                    window
                        .remove_event_listener_with_callback(
                            "keydown",
                            listener.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                }
            },
            (),
        )
    }

    let WordsContextInfo {
        columns, selected, ..
    } = &*word_info;
    html! {
        <div class="main">
            {columns.iter().enumerate().map(|(i, a)| {
                html! {
                    <Row row={a.clone()} is_selected={i == selected.0} />
                }
            }).collect::<Html>()}
        </div>
    }
}
