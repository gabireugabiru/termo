use yew::{function_component, html, Properties, Html};
use crate::key::Key;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub row: Vec<char>
}
#[function_component(Row)]
pub fn component(Props { row }: &Props) -> Html {
    html! {
        <div class="key_list">
        {row.iter().map(|a| {
            html! {
                <Key letter = {a.clone()} />
            }
        } ).collect::<Html>()}
        </div>
    }
}