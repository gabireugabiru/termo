use yew_router::Routable;
use yew::{html, Html};
use crate::main_app::MainApp;

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum AppRouter {
    #[at("/")]
    Home,
}

pub fn switch(route: &AppRouter) -> Html {
    match route {
        &AppRouter::Home => html!{ <MainApp />},
    }
}