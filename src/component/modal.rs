use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
  pub state: bool,
  pub modal_state_handle: Callback<MouseEvent>
}

#[function_component(TournamentModal)]
pub fn Modal(props: &ModalProps) -> Html {
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
        <button type="button" class="rounded bg-indigo-200 w-full" onclick={props.modal_state_handle.clone()}>{"생성"}</button>
      </div>
    </div>
  )
}
