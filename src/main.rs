use yew::prelude::*;

mod components;
use crate::components::input_row::InputRow;

static WORD_TO_FIND: &str = "funeraire";

#[derive(PartialEq, Clone)]
pub enum State {
    Good,
    Partial,
    Wrong,
}

impl State {
    fn to_string(&self) -> String {
        match self {
            State::Good => "good".to_string(),
            State::Partial => "partial".to_string(),
            State::Wrong => "wrong".to_string(),
        }
    }
}

#[derive(PartialEq, Clone)]
struct Progress {
    input: String,
    letter_state_list: Vec<State>,
}

#[function_component(App)]
fn app() -> Html {
    let progress: UseStateHandle<Vec<Progress>> = use_state(|| vec![]);
    let input: UseStateHandle<&str> = use_state(|| "");

    html! {
        <div>
            <h1>{"RUSTMO"}</h1>
            {for progress.iter().enumerate().map(|(_, progress)| html! {
                <InputRow
                    word_to_find={WORD_TO_FIND}
                    input={progress.input.clone()}
                    letter_state_list={progress.letter_state_list.clone()}
                />
            })}
            <InputRow
                word_to_find={WORD_TO_FIND}
                input={*input}
                letter_state_list={vec![]}
            />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
