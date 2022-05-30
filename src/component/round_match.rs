use std::collections::HashMap;

use yew::prelude::*;

use crate::types::Player;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub round_match: u8,
  pub players: HashMap<Option<Player>, u8>,
}

#[function_component(RoundMatch)]
pub fn round_match(props: &Props) -> Html {
  let players = props.players.iter().map(|player| {
    let result = match player.0 {
      Some(player) => html! { <div>{player.name.clone()}</div> },
      None => html! { <input /> },
    };
    
    result
  }).collect::<Vec<Html>>();
  html! {
    <div class="py-1">
      {players}
    </div>
  }
}