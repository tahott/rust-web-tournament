use std::collections::HashMap;

use yew::prelude::*;

use crate::types::Player;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub round_match: u16,
  pub players: HashMap<Player, u8>,
}

#[function_component(RoundMatch)]
pub fn round_match(props: &Props) -> Html {
  let players = props.players.iter().map(|player| {
    html! {
      <div class="flex flex-row">
        <div class="basis-2/3">{player.0.name.clone()}</div>
        <div class="basis-1/3">{player.1}</div>
      </div>
    }
  }).collect::<Vec<Html>>();
  html! {
    <div class="py-1">
      {players}
    </div>
  }
}