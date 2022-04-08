use crate::{
  context::WordsContextType,
  get_char_count,
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
        let key_state =
        if  word_info.completed[*row_selected] {

          let (part, part2) = word.split_at(i);

          let entered_chars_count1 = get_char_count(part.to_owned());
          let entered_chars_count2 = get_char_count(part2.to_owned());


          if let Some(word_char_count) = word_info.word_char_counts.get(a) {
            if *a == word[i] {
              KeyState::Exact
            }  else {
              let entered_chars1 = match entered_chars_count1.get(a) {
                Some(a) => a,
                None => &0
              };
              let entered_chars2 = match entered_chars_count2.get(a) {
                Some(a) => a,
                None => &0
              };
              gloo_console::log!(format!("{};{};{};{:?}/{:?}", word_char_count, entered_chars1, entered_chars2, part, part2));

              if word_char_count <= entered_chars1 || word_char_count <= entered_chars2 {
                KeyState::NotContain
              } else {
                gloo_console::log!("2");
                KeyState::Contain
              }


              // if let Some(entered_chars) = entered_chars_count1.get(a) {
              //   if word_char_count <= entered_chars {
              //     KeyState::NotContain
              //   } else {
              //     gloo_console::log!("3");
              //     KeyState::Contain
              //   }
              // } if let Some(entered_chars) =  entered_chars_count2.get(a)  {
              //   if word_char_count <= entered_chars {
              //     KeyState::NotContain
              //   } else {
              //     gloo_console::log!("2");

              //     KeyState::Contain
              //   }
              // }
            }

          } else {
            gloo_console::log!("1");
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
