use std::collections::HashMap;
use yew::prelude::*;

use crate::{component::RoundMatch, types::{Player}};

#[derive(Properties, PartialEq)]
pub struct Props {
  pub rounds: u8,
  pub matches: HashMap<u8, Vec<Player>>,
}

#[function_component(Round)]
pub fn round(props: &Props) -> Html {
 html!(
  html! {
    <div class="flex flex-col justify-evenly">
      {
        (0..props.rounds).collect::<Vec<u8>>().iter().map(|i| {
          let players = match props.matches.get(i) {
            Some(p) => p.clone(),
            None => Vec::with_capacity(2),
          };

          let mut player_hash_map = HashMap::new();
          if players.len() > 0 {
            players.iter().for_each(|p| {
              if p.name.len() > 0 {
                player_hash_map.insert(Player::new(&p.name), 0);
              } else {
                player_hash_map.insert(Player::new(""), 0);
              }
            });
          } else {
            player_hash_map.insert(Player::new(""), 0);
            player_hash_map.insert(Player::new(""), 0);
          }

          html! {
            <RoundMatch round_match={*i} players={player_hash_map} />
          }
        }).collect::<Vec<Html>>()
      }
    </div>
  }
 )
}