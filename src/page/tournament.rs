use std::{str::FromStr, collections::HashSet};

use gloo::utils::document;
use serde::{Deserialize, Serialize};
use uuid::{Uuid};
use wasm_bindgen::{JsCast};
use web_sys::{HtmlLiElement};
use yew::prelude::*;
use yew_router::{hooks::{use_location}, history::Location};

use crate::{component::Round, api::{get_tournament_detail}, types::{TournamentState, TournamentType, MatchEnums, Matches, TournamentStatus, Pariticipant}};

enum ActiveTab {
  Participant,
  Matches,
}

#[derive(Properties, PartialEq, Deserialize, Serialize)]
pub struct Props {
  pub participants: u16,
}

fn to_round(n: u16, count: u16, mut v: Vec<u16>) -> Vec<u16> {
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
    participant: Pariticipant::new(),
    title: String::from("title"),
    tournament_type: TournamentType::Tournament,
    matches: Matches::new(0),
    status: TournamentStatus::Prepare,
  });
  let cols = use_state(|| 0);
  let round = use_state(|| vec![]);
  let location = use_location().unwrap();
  let active_tab = use_state(|| ActiveTab::Matches);

  use_effect_with_deps({
    let tournament = tournament.clone();
    let cols = cols.clone();
    let round = round.clone();
    let uuid_str = location.pathname().split("/").last().unwrap().to_owned();

    move |_| {
      wasm_bindgen_futures::spawn_local(async move {
        let uuid_value = Uuid::from_str(&uuid_str).unwrap();
        let tournament_state = get_tournament_detail(uuid_value).await.unwrap();
        let mut partial_participants = HashSet::new();
  
        tournament.set(tournament_state.clone());

        let cols_vec = to_round(tournament_state.participant.count, 1, vec![]);
        cols.set(cols_vec.len() as u16);
        round.set(cols_vec.clone().iter().map(|r| {
          let matches = tournament_state.matches.clone();

          let players = match r {
            128 => (matches.0).get(&MatchEnums::RoundOf256).unwrap().clone(),
            64 => (matches.0).get(&MatchEnums::RoundOf128).unwrap().clone(),
            32 => (matches.0).get(&MatchEnums::RoundOf64).unwrap().clone(),
            16 => (matches.0).get(&MatchEnums::RoundOf32).unwrap().clone(),
            8 => (matches.0).get(&MatchEnums::RoundOf16).unwrap().clone(),
            4 => (matches.0).get(&MatchEnums::QuarterFinal).unwrap().clone(),
            2 => (matches.0).get(&MatchEnums::SemiFinal).unwrap().clone(),
            1 => (matches.0).get(&MatchEnums::Final).unwrap().clone(),
            _ => (matches.0).get(&MatchEnums::Final).unwrap().clone(),
          };

          players.clone().into_iter().for_each(|f| {
            f.1.clone().into_iter().for_each(|player| {
              partial_participants.insert(player);
            });
          });

          html! {
            <Round rounds={*r} matches={players.clone()} />
          }
        }).collect::<Vec<Html>>());
      });
      || ()
    }
  }, ());

  let onclick = |tab: ActiveTab| {
    let active_tab = active_tab.clone();


    Callback::from(move |e: MouseEvent| {
      let t_nav = document().get_elements_by_class_name("t-nav");

      for i in 0..t_nav.length() {
        let nav = t_nav.get_with_index(i).unwrap().dyn_into::<HtmlLiElement>().unwrap();
        nav.set_class_name("t-nav mr-2");
      };

      match tab {
        ActiveTab::Participant => {
          let target = e.target().unwrap().dyn_into::<HtmlLiElement>().unwrap();
          target.set_class_name("t-nav mr-2 text-yellow-500");
          active_tab.set(ActiveTab::Participant);
        },
        ActiveTab::Matches => {
          let target = e.target().unwrap().dyn_into::<HtmlLiElement>().unwrap();
          target.set_class_name("t-nav mr-2 text-yellow-500");
          active_tab.set(ActiveTab::Matches);
        },
      }
    })
  };

  html!(
    <div class="container mx-auto m-2 p-4">
      <div class="border-b border-gray-200 dark:border-gray-700">
        <ul class="flex flex-wrap -mb-px text-sm font-medium text-center text-gray-500 dark:text-gray-400 cursor-pointer">
          <li class="t-nav mr-2" onclick={onclick(ActiveTab::Participant)}>
            {"참가자"}
          </li>
          <li class="t-nav mr-2 text-yellow-500" onclick={onclick(ActiveTab::Matches)}>
            {"대진표"}
          </li>
        </ul>
      </div>
      {
        match *active_tab {
          ActiveTab::Participant => html! {
            <div class="grid grid-cols-8 gap-4 mt-2">
              {
                (*tournament.participant.list).iter().map(|f| {
                  match f {
                    Some(player) => {
                      html! {
                        <div>{player.name.clone()}</div>
                      }
                    },
                    None => {
                      html! {
                        <div class="flex flex-row border border-solid rounded-md p-1 items-center">
                          <input class="w-full border-none grow-[2] focus:outline-none" />
                          <input type="checkbox" role="switch" />
                        </div>
                      }
                    },
                  }
                }).collect::<Vec<Html>>()
              }
            </div>
          },
          ActiveTab::Matches => html! {
            <div class={format!("grid grid-cols-[repeat({},_minmax(0,1fr))] gap-4 bg-sky-100", *cols)}>
              {(*round).clone()}
            </div>
          },
        }
      }
    </div>
  )
}