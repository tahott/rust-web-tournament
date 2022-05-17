use std::rc::Rc;
use yew::prelude::*;

use crate::component::TournamentModal;

enum StateAction {
  Open,
  Close,
}
struct ModalState {
  is_open: bool
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

  html!(
    <div class="py-2">
      <div class="container mx-auto h-screen">
        {"진행중인 토너먼트 리스트"}
        <div class="flex justify-end">
          <button type="button" class="rounded text-white bg-sky-700 px-2 py-1.75" {onclick}>
            {"토너먼트 생성"}
          </button>
        </div>
        <TournamentModal state={modal_state_handle.is_open} />
        // Grid
      </div>
    </div>
  )
}
