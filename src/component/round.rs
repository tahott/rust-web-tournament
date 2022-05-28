use std::collections::HashMap;

use yew::prelude::*;

use crate::component::RoundMatch;

use super::round_match::Player;

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
          let player_hash_map = HashMap::from([
            (Player::new("홍길동"), 0 as u8),
            (Player::new(""), 0 as u8),
          ]);

          html! {
            <RoundMatch round_match={*i} players={player_hash_map} />
          }
        }).collect::<Vec<Html>>()
      }
    </div>
  }
 )
}