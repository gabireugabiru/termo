use std::collections::HashMap;

use rand::Rng;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::MouseEvent;
use yew::{
  function_component, html, use_effect_with_deps, use_reducer, Children,
  ContextProvider, Properties, Reducible, UseReducerHandle,
};

use crate::{get_char_count, WORDS};
#[derive(PartialEq, Properties)]
pub struct Props {
  pub children: Children,
}
#[derive(Clone, PartialEq)]
pub struct WordsContextInfo {
  pub word: String,
  pub word_char_counts: HashMap<char, u32>,
  pub height: usize,
  pub columns: Vec<Vec<char>>,
  pub selected: (usize, usize),
  pub completed: Vec<bool>,
  pub finished: bool,
  pub win: bool,
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
  pub fn from(height: usize) -> Self {
    let random = rand::thread_rng().gen_range(0..WORDS.len());
    let word = WORDS[random].to_owned();
    let word_char_counts = get_char_count(word.chars().collect());
    let rows = word.len();
    let mut column_vec = Vec::new();
    let mut row_vec = Vec::new();
    let mut completed = Vec::new();
    for _ in 0..rows {
      row_vec.push(' ');
    }
    for _ in 0..height {
      column_vec.push(row_vec.clone());
      completed.push(false);
    }

    WordsContextInfo {
      columns: column_vec,
      height,
      word_char_counts,
      word,
      completed,
      win: false,
      finished: false,
      selected: (0, 0),
    }
  }
}
pub enum WordAction {
  Push(char),
  Back,
  New,
  SetColumn(usize),
  Move(i8),
  Reset,
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
          gloo_console::log!(format!(
            "{}, {}",
            self.selected.1,
            self.word.len()
          ));
          let r#char = if self.selected.1 == self.word.len() {
            ' '
          } else {
            self.columns[self.selected.0][self.selected.1]
          };
          if r#char == ' ' {
            new_data.columns[self.selected.0][self.selected.1 - 1] = ' ';
            new_data.selected.1 -= 1;
          } else {
            new_data.columns[self.selected.0][self.selected.1] = ' ';
          }
        }
      }
      Self::Action::New => {
        if self.is_last() {
          new_data.completed[self.selected.0] = true;
          new_data.selected.0 += 1;

          let guess =
            self.columns[self.selected.0].iter().collect::<String>();

          if guess == self.word {
            new_data.finished = true;
            new_data.win = true;
          } else if new_data.selected.0 == self.height {
            new_data.finished = true;
            new_data.win = false;
          }
          new_data.selected.1 = 0;
        }
      }
      Self::Action::SetColumn(i) => {
        new_data.selected.1 = i;
      }
      Self::Action::Move(s) => {
        if s < 0 {
          if self.selected.1 > 0 {
            new_data.selected.1 -= 1;
          }
        } else {
          if self.selected.1 < self.word.len() {
            new_data.selected.1 += 1;
          }
        }
      }
      Self::Action::Reset => {
        new_data = WordsContextInfo::from(5);
      }
    };
    new_data.into()
  }
}

pub type WordsContextType = UseReducerHandle<WordsContextInfo>;

#[function_component(WordsContext)]
pub fn context(Props { children }: &Props) -> Html {
  let state = use_reducer(|| WordsContextInfo::from(5));

  // use_effect_with_deps(
  //   |_| {
  //     let listener =
  //       Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |ev| {
  //         ev.prevent_default();
  //       }));
  //     let window = web_sys::window().unwrap();
  //     window
  //       .add_event_listener_with_callback(
  //         "contextmenu",
  //         listener.as_ref().unchecked_ref(),
  //       )
  //       .unwrap();

  //     move || {
  //       window
  //         .remove_event_listener_with_callback(
  //           "contextmenu",
  //           listener.as_ref().unchecked_ref(),
  //         )
  //         .unwrap();
  //     }
  //   },
  //   (),
  // );
  html! {
    <ContextProvider<WordsContextType> context={state.clone()}>
      {for children.iter()}
    </ContextProvider<WordsContextType> >
  }
}
