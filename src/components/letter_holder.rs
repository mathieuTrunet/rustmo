use crate::State;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub letter: char,
    pub state: State,
}

#[function_component]
pub fn LetterHolder(props: &Props) -> Html {
    html! { <span class={props.state.to_string()}>{props.letter}</span> }
}
