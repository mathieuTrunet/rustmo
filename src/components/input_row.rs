use crate::{components::letter_holder::LetterHolder, State};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub word_to_find: String,
    pub input: String,
    pub letter_state_list: Vec<State>,
}

#[function_component]
pub fn InputRow(props: &Props) -> Html {
    let mut chars: std::str::Chars = props.word_to_find.chars();

    let first_letter: char = chars.next().unwrap_or_default();

    let rest_of_the_word: std::str::Chars = chars;

    html! {
        <div>
        <LetterHolder
            state={State::Good}
            letter={first_letter}
        />
        {for rest_of_the_word.into_iter().enumerate().map(|(index, _)| html!{
            <LetterHolder
                state={props.letter_state_list.get(index).unwrap_or(&State::Wrong).clone()}
                letter={props.input.chars().nth(index).unwrap_or('_')}
            />
        })}
        </div>
    }
}
