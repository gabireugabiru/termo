use yew::function_component;
use yew_router::{BrowserRouter, Switch};
use yew::html;
use router::{AppRouter, switch};
use crate::context::WordsContext;
mod main_app;
mod row;
mod key;
mod context;
mod router;


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