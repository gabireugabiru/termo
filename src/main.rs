use std::collections::HashMap;

use crate::{context::WordsContext, popup::PopUpContext};
use router::{switch, AppRouter};
use yew::function_component;
use yew::html;
use yew_router::{BrowserRouter, Switch};
mod context;
mod key;
mod main_app;
mod popup;
mod router;
mod row;

pub const WORDS: [&str; 5] =
  ["teste", "online", "caralho", "joaoralho", "senhoralho"];

pub fn get_char_count(word: Vec<char>) -> HashMap<char, u32> {
  gloo_console::log!("here");
  let mut hash: HashMap<char, u32> = HashMap::new();
  for i in word {
    match hash.clone().get(&i) {
      Some(a) => {
        hash.insert(i, a + 1);
      }
      None => {
        hash.insert(i, 1);
      }
    }
  }
  hash
}
pub fn exact_char_count(
  word: Vec<char>,
  entered: Vec<char>,
) -> HashMap<char, u32> {
  let mut hash = HashMap::new();
  for (i, c) in entered.iter().enumerate() {
    if *c == word[i] {
      match hash.clone().get(c) {
        Some(a) => {
          hash.insert(*c, a + 1);
        }
        None => {
          hash.insert(*c, 1);
        }
      }
    }
  }
  hash
}

#[function_component(Main)]
fn app() -> Html {
  html! {
    <WordsContext>
      <PopUpContext>
        <BrowserRouter>
         <Switch<AppRouter> render={Switch::render(switch)} />
        </BrowserRouter>
      </PopUpContext>
    </WordsContext>


  }
}
fn main() {
  yew::start_app::<Main>();
}
