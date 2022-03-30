use crate::main_app::MainApp;
use yew::{html, Html};
use yew_router::Routable;

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum AppRouter {
  #[at("/")]
  Home,
}

pub fn switch(route: &AppRouter) -> Html {
  match route {
    &AppRouter::Home => html! { <MainApp />},
  }
}
