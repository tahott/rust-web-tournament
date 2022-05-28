use std::collections::HashMap;

use yew::prelude::*;

#[derive(Hash, PartialEq, Eq)]
pub struct Player {
  name: String,
}

impl Player {
  pub fn new(name: &str) -> Option<Player> {
    if name.len() > 0 {
      return Some(Player { name: String::from(name) })
    } else {
      return None
    }
  }
}

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