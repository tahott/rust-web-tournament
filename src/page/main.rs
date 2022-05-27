use std::{rc::Rc};
use yew::prelude::*;
use yew_router::components::Link;

use crate::api::get_tournament_list;
use crate::route::Route;
use crate::{component::{TournamentModal, TournamentCard}};

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

#[function_component(Main)]
pub fn main() -> Html {
  let tournament = use_state(|| vec![]);
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

  use_effect_with_deps({
    let tournament = tournament.clone();

    move |_| {
      wasm_bindgen_futures::spawn_local(async move {
        let tournament_summary = get_tournament_list().await.unwrap();
        
        tournament.set(tournament_summary.clone().iter().map(|t| {
          html! {
            <Link<Route> to={Route::TournamentPage { id: t.id }}>
              <TournamentCard title={t.title.clone()} status={t.status.clone()} />
            </Link<Route>>
          }
        }).collect::<Vec<Html>>());
      });
      || ()
    }
  }, ());

  html!(
    <div class="py-2">
      <div class="container mx-auto h-screen">
        {"진행중인 토너먼트 리스트"}
        <div class="flex justify-end">
          <button type="button" class="rounded text-white bg-sky-700 px-2 py-1.75" {onclick}>
            {"토너먼트 생성"}
          </button>
        </div>
        <div class="flex flex-wrap gap-4">
          {(*tournament).clone()}
        </div>
        <TournamentModal state={modal_state_handle.is_open} />
      </div>
    </div>
  )
}