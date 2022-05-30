use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::{Uuid};
use yew::prelude::*;
use yew_router::{hooks::{use_location}, history::Location};

use crate::{component::Round, api::{get_tournament_detail}, types::{TournamentState, TournamentType, MatchEnums}};

#[derive(Properties, PartialEq, Deserialize, Serialize)]
pub struct Props {
  pub participants: u8,
}

fn to_round(n: u8, count: u8, mut v: Vec<u8>) -> Vec<u8> {
  v.push(n / 2);
  match n > 2 {
    true => {
      to_round(n / 2, count + 1, v)
    },
    false => {
      v
    }
  }
}

#[function_component(Tournament)]
pub fn tournament() -> Html {
  let tournament = use_state(|| TournamentState {
    id: Uuid::new_v4(),
    participants: 0,
    title: String::from("title"),
    tournament_type: TournamentType::Tournament,
    matches: None,
  });
  let cols = use_state(|| 0);
  let round = use_state(|| vec![]);
  let location = use_location().unwrap();

  use_effect_with_deps({
    let tournament = tournament.clone();
    let cols = cols.clone();
    let round = round.clone();
    let uuid_str = location.pathname().split("/").last().unwrap().to_owned();

    move |_| {
      wasm_bindgen_futures::spawn_local(async move {
        let uuid_value = Uuid::from_str(&uuid_str).unwrap();
        let tournament_state = get_tournament_detail(uuid_value).await.unwrap();
  
        tournament.set(tournament_state.clone());

        let cols_vec = to_round(tournament_state.participants, 1, vec![]);
        cols.set(cols_vec.len() as u8);
        round.set(cols_vec.clone().iter().map(|r| {
          let m = tournament_state.matches.clone().unwrap();
          let a = match r {
            8 => m.get(&MatchEnums::RoundOf16).unwrap().clone(),
            4 => m.get(&MatchEnums::QuarterFinal).unwrap().clone(),
            2 => m.get(&MatchEnums::SemiFinal).unwrap().clone(),
            1 => m.get(&MatchEnums::Final).unwrap().clone(),
            _ => None,
          }.unwrap();

          html! {
            <Round rounds={*r} matches={a.clone()} />
          }
        }).collect::<Vec<Html>>());
      });
      || ()
    }
  }, ());

  html!(
    <div class="container mx-auto m-2 p-4 bg-sky-100">
      <div class={format!("grid grid-cols-[repeat({},_minmax(0,1fr))] gap-4", *cols)}>
        {(*round).clone()}
      </div>
    </div>
  )
}