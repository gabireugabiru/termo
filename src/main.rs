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
