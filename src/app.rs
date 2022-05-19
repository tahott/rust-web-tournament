use std::{rc::Rc, str::FromStr};
use gloo::utils::document;
use serde::Serialize;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{HtmlSelectElement, HtmlInputElement, console};
use yew::prelude::*;

use crate::component::TournamentModal;

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

enum StateAction {
  Open,
  Close,
}
struct ModalState {
  pub is_open: bool
}

impl Reducible for ModalState {
  type Action = StateAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      StateAction::Open => Self { is_open: true }.into(),
      StateAction::Close => Self { is_open: false }.into(),
    }
  }
}

#[function_component(App)]
pub fn app() -> Html {
  let modal_state_handle = use_reducer(|| ModalState { is_open: false });
  
  let onclick = {
    let modal_state_handle = modal_state_handle.clone();

    if !modal_state_handle.is_open {
      Callback::from(move |_| {
        modal_state_handle.dispatch(StateAction::Open);
      })
    } else {
      Callback::from(move |_| {
        modal_state_handle.dispatch(StateAction::Close);
      })
    }
  };

  let change_modal_state = {
    let modal_state_handle = modal_state_handle.clone();

    if modal_state_handle.is_open == false {
      Callback::from(move |_| {
        modal_state_handle.dispatch(StateAction::Open);
      })
    } else {
      Callback::from(move |_| {
        let val = parse_element::<HtmlSelectElement>("tournament_type").value();
        let participants = parse_element::<HtmlInputElement>("participants").value_as_number();
        let title = parse_element::<HtmlInputElement>("title").value();

        let tournament = TournamentState {
          tournament_type: TournamentType::from_str(&val).unwrap(),
          participants: participants as u8,
          title,
        };

        console::log_1(&JsValue::from_serde(&tournament).unwrap());
        modal_state_handle.dispatch(StateAction::Close);
      })
    }
  };

  html!(
    <div class="py-2">
      <div class="container mx-auto h-screen">
        {"진행중인 토너먼트 리스트"}
        <div class="flex justify-end">
          <button type="button" class="rounded text-white bg-sky-700 px-2 py-1.75" {onclick}>
            {"토너먼트 생성"}
          </button>
        </div>
        <TournamentModal state={modal_state_handle.is_open} modal_state_handle={change_modal_state} />
        // Grid
      </div>
    </div>
  )
}

fn parse_element<T: wasm_bindgen::JsCast>(id: &str) -> T {
  document().get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
}
