use std::str::FromStr;

use gloo::utils::document;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::{Route};

#[derive(Properties, PartialEq)]
pub struct ModalProps {
  pub state: bool,
}

#[derive(Serialize)]
enum TournamentType {
  #[serde(rename="tournament")]
  Tournament,
  #[serde(rename="swissLeague")]
  SwissLeague,
}

impl FromStr for TournamentType {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "tournament" => Ok(TournamentType::Tournament),
      "swiss_league" => Ok(TournamentType::SwissLeague),
      _ => Err(()),
    }
  }
}

#[derive(Serialize)]
struct TournamentState {
  #[serde(rename="tournamentType")]
  tournament_type: TournamentType,
  participants: u8,
  title: String,
}

#[function_component(TournamentModal)]
pub fn Modal(props: &ModalProps) -> Html {
  let history = use_history().unwrap();
  let change_modal_state =  {
    Callback::once(move |_| {
      let val = parse_element::<HtmlSelectElement>("tournament_type").value();
      let participants = parse_element::<HtmlInputElement>("participants").value_as_number();
      let title = parse_element::<HtmlInputElement>("title").value();
  
      let tournament = TournamentState {
        tournament_type: TournamentType::from_str(&val).unwrap(),
        participants: participants as u8,
        title,
      };
  
      history.push_with_state(Route::TournamentPage { id: "abc".to_string() }, tournament).unwrap()
    })
  };

  html!(
    <div class={if props.state {"max-w-lg w-full absolute mx-auto p-4 text-white rounded-lg shadow-2xl bg-indigo-500 shadow-indigo-500/50"} else {"hidden"}}>
      <div class="grid grid-cols-2 gap-2 mb-6">
        <select id="tournament_type" class="bg-indigo-400">
          <option value="tournament" selected={true}>{"토너먼트"}</option>
          <option value="swiss_league" disabled={true}>{"스위스 리그"}</option>
        </select>
        <input id="participants" class="bg-indigo-400 rounded pl-2" type="number" min=2 max=64 value=2 />
      </div>
      <div>
        <input id="title" class="rounded w-full pl-2 mb-6 bg-indigo-400" type="text" placeholder="title" />
      </div>
      <div>
        <button type="button" class="rounded bg-indigo-200 w-full" onclick={change_modal_state}>{"생성"}</button>
      </div>
    </div>
  )
}

fn parse_element<T: wasm_bindgen::JsCast>(id: &str) -> T {
  document().get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
}
