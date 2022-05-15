use yew::prelude::*;

use crate::component::RoundMatch;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub matches: u8
}

#[function_component(Round)]
pub fn round(props: &Props) -> Html {
 html!(
  html! {
    <div class="flex flex-col justify-evenly">
      {
        (1..=props.matches).collect::<Vec<u8>>().iter().map(|i| {
          html! {
            <RoundMatch round_match={*i} />
          }
        }).collect::<Vec<Html>>()
      }
    </div>
  }
 )
}