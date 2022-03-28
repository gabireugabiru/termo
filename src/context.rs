use rand::Rng;
use yew::{
    function_component, html, use_effect_with_deps, use_reducer, Children,
    ContextProvider, Properties, Reducible, UseReducerHandle,
};

use crate::WORDS;
#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}
#[derive(Clone, PartialEq)]
pub struct WordsContextInfo {
    pub word: String,
    pub columns: Vec<Vec<char>>,
    pub selected: (usize, usize),
}

pub enum WordAction {
    Push(char),
    Back,
}

impl Reducible for WordsContextInfo {
    type Action = WordAction;
    fn reduce(
        self: std::rc::Rc<Self>,
        action: Self::Action,
    ) -> std::rc::Rc<Self> {
        let mut new_data = (*self).clone();
        match action {
            Self::Action::Push(char) => {
                if self.selected.1 < self.word.len() {
                    new_data.columns[self.selected.0][self.selected.1] =
                        char;
                    new_data.selected.1 += 1;
                }
            }
            Self::Action::Back => {
                if self.selected.1 > 0 {
                    if self.selected.1 != self.word.len() {
                        new_data.columns[self.selected.0]
                            [self.selected.1] = ' ';
                    }
                    new_data.selected.1 -= 1;
                } else {
                    new_data.columns[self.selected.0][self.selected.1] =
                        ' ';
                }
            }
        };
        new_data.into()
    }
}

pub type WordsContextType = UseReducerHandle<WordsContextInfo>;

#[function_component(WordsContext)]
pub fn context(Props { children }: &Props) -> Html {
    let state = use_reducer(|| {
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
            word,
            selected: (0, 0),
        }
    });
    {
        use_effect_with_deps(|i| || {}, state.clone())
    }
    html! {
        <ContextProvider<WordsContextType> context={state.clone()}>
            {for children.iter()}
        </ContextProvider<WordsContextType> >
    }
}
