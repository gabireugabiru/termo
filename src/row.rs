use crate::{
  context::WordsContextType,
  exact_char_count,
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
      {
        row.iter().enumerate().map(|(i, a)| {
        let word = word_info.word.chars().collect::<Vec<char>>();
        let entered_word = word_info.columns[*row_selected].clone();
        let entered_char_exacts = exact_char_count(word.clone(), entered_word);
        let key_state =
        if  word_info.completed[*row_selected] {
          if let Some(word_char_count) = word_info.word_char_counts.get(a) {
            if *a == word[i] {
              KeyState::Exact
            } else if let Some(entered_char_exact) = entered_char_exacts.get(a)  {
              if word_char_count <= entered_char_exact {
                KeyState::NotContain
              } else {
                KeyState::Contain
              }
            } else {
              KeyState::Contain
            }
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
