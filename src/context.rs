use rand::Rng;
use yew::{function_component, html, ContextProvider, Children, Properties, use_state};

use crate::WORDS;
#[derive(PartialEq,Properties)]
pub struct Props {
    pub children: Children
}
#[derive(Clone, PartialEq)]
pub struct WordsContextInfo {
    pub word: String,
    pub columns: Vec<Vec<char>>
}

#[function_component(WordsContext)]
pub fn context(Props {children}: &Props) -> Html {
    let state = use_state(|| {
        let random = rand::thread_rng().gen_range(0..WORDS.len());
        let word = WORDS[random].to_owned();
        let columns = 4;
        let rows = word.len();
        let mut column_vec = Vec::new();
        let mut row_vec = Vec::new();
        for _ in 0..rows {
            row_vec.push(' ');
        }
        for _ in 0..columns {
            column_vec.push(row_vec.clone());
        }
      
        WordsContextInfo {
            columns: column_vec,
            word
        }
    });
    html! {
        <ContextProvider<WordsContextInfo> context={(*state).clone()}>
            {for children.iter()}
        </ContextProvider<WordsContextInfo> >
    }
}