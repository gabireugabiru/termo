use yew::{
  function_component, html, use_context, use_effect_with_deps,
  use_reducer, Callback, Children, ContextProvider, Html, Properties,
  Reducible, UseReducerHandle,
};

use crate::context::{WordAction, WordsContextType};
#[derive(PartialEq, Properties)]
pub struct Props {
  pub children: Children,
}
#[derive(PartialEq, Clone)]
pub struct PopUpContextInfo {
  open: bool,
  html: Html,
}
impl PopUpContextInfo {
  pub fn new() -> Self {
    Self {
      html: html! {
        <span class="win">
          {"you win mothafucka"}
        </span>
      },
      open: false,
    }
  }
}
pub enum PopUpAction {
  Open(Html),
  Close,
}

impl Reducible for PopUpContextInfo {
  type Action = PopUpAction;
  fn reduce(
    self: std::rc::Rc<Self>,
    action: Self::Action,
  ) -> std::rc::Rc<Self> {
    let mut new_data = (*self).clone();
    match action {
      Self::Action::Open(html) => {
        new_data.html = html;
        new_data.open = true;
      }
      Self::Action::Close => {
        new_data = PopUpContextInfo::new();
      }
    };
    new_data.into()
  }
}

pub type PopUpContextType = UseReducerHandle<PopUpContextInfo>;
#[function_component(PopUpContext)]
pub fn context(Props { children }: &Props) -> Html {
  let state = use_reducer(|| PopUpContextInfo::new());
  let word_info = use_context::<WordsContextType>().unwrap();
  {
    let state = state.clone();
    use_effect_with_deps(
      move |(finished, win)| {
        gloo_console::log!("teste1");
        if *finished {
          state.dispatch(PopUpAction::Open(html! {
            <span class={format!("{}", if *win { "win" } else { "lose" })}>
              {if *win {"You win"} else { "You lose"} }
            </span>
          }));
        }
        || {}
      },
      (word_info.finished.clone(), word_info.win.clone()),
    )
  }

  let reset_game = {
    let word_info = word_info.clone();
    let state = state.clone();
    Callback::from(move |_| {
      state.dispatch(PopUpAction::Close);
      word_info.dispatch(WordAction::Reset);
    })
  };
  html! {
    <ContextProvider<PopUpContextType> context={state.clone()}>
      {if state.open {
        html!{
          <div id="pop_up">
            <div>
              {  state.html.clone() }
              <button onclick={reset_game} class="play_again">
                {"Play again"}
              </button>
            </div>
          </div>
        }
      } else {
        html! {}
      }}
      {for children.iter()}
    </ContextProvider<PopUpContextType>>
  }
}
