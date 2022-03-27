use yew::{function_component, html, Properties};
#[derive(PartialEq, Properties)]
pub struct Props {
    pub letter: char,
}
#[function_component(Key)]
pub fn component(Props { letter }: &Props) -> Html {
    html! {
        <div class="key">
            {letter}
        </div>
    }
}