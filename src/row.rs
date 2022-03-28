use crate::{
    context::{WordsContextInfo, WordsContextType},
    key::Key,
};
use yew::{function_component, html, use_context, Html, Properties};
#[derive(Properties, PartialEq)]
pub struct Props {
    pub row: Vec<char>,
    pub is_selected: bool,
}
#[function_component(Row)]
pub fn component(Props { row, is_selected }: &Props) -> Html {
    let word_info = use_context::<WordsContextType>().unwrap();
    html! {
        <div class="key_list">
        {row.iter().enumerate().map(|(i, a)| {
            html! {
                <Key is_selected={*is_selected && i == word_info.selected.1} letter = {a.clone()} />
            }
        } ).collect::<Html>()}
        </div>
    }
}
