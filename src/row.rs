use crate::{
  context::{WordsContextInfo, WordsContextType},
  key::{Key, KeyState},
};
use yew::{function_component, html, use_context, Html, Properties};
#[derive(Properties, PartialEq)]
pub struct Props {
  pub row: Vec<char>,
  pub is_selected: bool,
  pub row_selected: usize,
}
#[function_component(Row)]
pub fn component(
  Props {
    row,
    is_selected,
    row_selected,
  }: &Props,
) -> Html {
  let word_info = use_context::<WordsContextType>().unwrap();
  html! {
    <div class="key_list">
      {row.iter().enumerate().map(|(i, a)| {
        let key_state = if word_info.completed[*row_selected] {
          let word = word_info.word.chars().collect::<Vec<char>>();
          if word[i] == *a {
            KeyState::Exact
          } else if word.contains(a) {
            KeyState::Contain
          } else {
            KeyState::NotContain
          }

        } else {
          KeyState::Default
        };
        html! {
          <Key
          is_selected={*is_selected && i == word_info.selected.1}
          column={i}
          letter = {a.clone()}
          state={key_state}
          />
        }
      } ).collect::<Html>()}
    </div>
  }
}
