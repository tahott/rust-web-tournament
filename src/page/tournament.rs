use serde::{Deserialize, Serialize};
use uuid::{Uuid, uuid};
use yew::prelude::*;

use crate::{component::Round, api::{get_tournament_detail}, types::{TournamentState, TournamentType}};

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
  let tournament = use_state(|| TournamentState { id: Uuid::new_v4(), participants: 0, title: String::from("title"), tournament_type: TournamentType::Tournament});
  let cols = use_state(|| 0);
  let round = use_state(|| vec![]);

  use_effect_with_deps({
    let tournament = tournament.clone();
    let cols = cols.clone();
    let round = round.clone();

    move |_| {
      wasm_bindgen_futures::spawn_local(async move {
        let tournament_state = get_tournament_detail(uuid!("dee91c81-598f-48b7-a20e-7b5246f4aa10")).await.unwrap();
  
        tournament.set(tournament_state.clone());

        let cols_vec = to_round(tournament_state.participants, 1, vec![]);
        cols.set(cols_vec.len() as u8);
        round.set(cols_vec.clone().iter().map(|r| {
          html! {
            <Round matches={*r} />
          }
        }).collect::<Vec<Html>>());
      });
      || ()
    }
  }, ());

  html!(
    <div class="container mx-auto bg-sky-500">
      <div class={format!("grid grid-cols-[repeat({},_minmax(0,1fr))] gap-4", *cols)}>
        {(*round).clone()}
      </div>
    </div>
  )
}