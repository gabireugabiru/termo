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

fn sla() {
  let a = String::from("bom dia");
}
