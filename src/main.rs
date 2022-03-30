use crate::context::WordsContext;
use router::{switch, AppRouter};
use yew::function_component;
use yew::html;
use yew_router::{BrowserRouter, Switch};
mod context;
mod key;
mod main_app;
mod router;
mod row;

pub const WORDS: [&str; 3] = ["teste", "online", "doismilevinte"];

#[function_component(Main)]
fn app() -> Html {
  html! {
    <WordsContext>
      <BrowserRouter>
        <Switch<AppRouter> render={Switch::render(switch)} />
      </BrowserRouter>
    </WordsContext>


  }
}
fn main() {
  yew::start_app::<Main>();
}

fn sla() {
  let a = String::from("bom dia");
}
