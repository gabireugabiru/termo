use yew::{function_component, html, use_context, Callback, Properties};

use crate::context::{WordAction, WordsContextType};
#[derive(PartialEq, Eq)]
pub enum KeyState {
  Contain,
  Exact,
  NotContain,
  Default,
}

#[derive(PartialEq, Properties)]
pub struct Props {
  pub letter: char,
  pub is_selected: bool,
  pub column: usize,
  pub state: KeyState,
}
#[function_component(Key)]
pub fn component(
  Props {
    letter,
    is_selected,
    column,
    state,
  }: &Props,
) -> Html {
  let word_info = use_context::<WordsContextType>().unwrap();
  let onclick = {
    let column = column.clone();
    Callback::from(move |_| {
      word_info.dispatch(WordAction::SetColumn(column));
    })
  };
  let mut class = vec!["key"];
  if *is_selected {
    class.push("selected");
  }
  if *state == KeyState::Contain {
    class.push("contain");
  }
  if *state == KeyState::Exact {
    class.push("exact");
  }
  if *state == KeyState::NotContain {
    class.push("not_contain");
  }
  html! {
    <div onclick={onclick} class={class}>
      {letter}
    </div>
  }
}
