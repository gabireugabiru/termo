use yew::{function_component, html, use_context, Html};
use crate::{row::Row, context::WordsContextInfo};
#[function_component(MainApp)] 
pub fn component() -> Html {
    let WordsContextInfo {columns, .. } = use_context::<WordsContextInfo>().unwrap();

    html! {
        <div class="main">
            {columns.iter().map(|a| {
                html! {
                    <Row row={a.clone()} />
                }
            }).collect::<Html>()}
        </div>
    }
}