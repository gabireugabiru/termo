use std::vec;

use rand::Rng;
use yew::{
  function_component, html, use_effect_with_deps, use_reducer, Children,
  ContextProvider, Properties, Reducible, UseReducerHandle,
};
use yew_router::history::History;

use crate::WORDS;
#[derive(PartialEq, Properties)]
pub struct Props {
  pub children: Children,
}
#[derive(Clone, PartialEq)]
pub struct WordsContextInfo {
  pub word: String,
  pub height: usize,
  pub columns: Vec<Vec<char>>,
  pub selected: (usize, usize),
  pub completed: Vec<bool>,
}
impl WordsContextInfo {
  pub fn is_last(&self) -> bool {
    gloo_console::log!(format!(
      "{}, {}",
      self.selected.1,
      self.word.len()
    ));
    if self.selected.1 == self.word.len() {
      true
    } else {
      false
    }
  }
}
pub enum WordAction {
  Push(char),
  Back,
  New,
  SetColumn(usize),
}

impl Reducible for WordsContextInfo {
  type Action = WordAction;
  fn reduce(
    self: std::rc::Rc<Self>,
    action: Self::Action,
  ) -> std::rc::Rc<Self> {
    let mut new_data = (*self).clone();
    match action {
      Self::Action::Push(char) => {
        if !self.is_last() {
          new_data.columns[self.selected.0][self.selected.1] = char;
          new_data.selected.1 += 1;
        }
      }
      Self::Action::Back => {
        if self.selected.1 > 0 {
          new_data.columns[self.selected.0][self.selected.1 - 1] = ' ';
          new_data.selected.1 -= 1;
        }
      }
      Self::Action::New => {
        if self.is_last() {
          new_data.completed[self.selected.0] = true;
          new_data.selected.0 += 1;
          let mut guess = String::new();
          for i in &self.columns[self.selected.0] {
            guess.push(*i);
          }

          // let guess = guess
          if new_data.selected.0 == self.height || guess == self.word {
            let window = web_sys::window().unwrap();
            window.alert_with_message("game ended".into());
          }
          new_data.selected.1 = 0;
        }
      }
      Self::Action::SetColumn(i) => {
        new_data.selected.1 = i;
      }
    };
    new_data.into()
  }
}

pub type WordsContextType = UseReducerHandle<WordsContextInfo>;

#[function_component(WordsContext)]
pub fn context(Props { children }: &Props) -> Html {
  let state = use_reducer(|| {
    let columns = 4;
    let random = rand::thread_rng().gen_range(0..WORDS.len());
    let word = WORDS[random].to_owned();
    let rows = word.len();
    let mut column_vec = Vec::new();
    let mut row_vec = Vec::new();
    let mut completed = Vec::new();
    for _ in 0..rows {
      row_vec.push(' ');
    }
    for _ in 0..columns {
      column_vec.push(row_vec.clone());
      completed.push(false);
    }

    WordsContextInfo {
      columns: column_vec,
      height: columns,
      word,
      completed,
      selected: (0, 0),
    }
  });
  {
    use_effect_with_deps(|i| || {}, state.clone())
  }
  html! {
    <ContextProvider<WordsContextType> context={state.clone()}>
      {for children.iter()}
    </ContextProvider<WordsContextType> >
  }
}
