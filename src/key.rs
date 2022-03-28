use yew::{function_component, html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub letter: char,
    pub is_selected: bool,
}
#[function_component(Key)]
pub fn component(
    Props {
        letter,
        is_selected,
    }: &Props,
) -> Html {
    let class = if *is_selected { "key selected" } else { "key" };
    html! {
        <div class={class}>
            {letter}
        </div>
    }
}
