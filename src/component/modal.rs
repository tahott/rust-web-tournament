use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
  pub state: bool
}

#[function_component(TournamentModal)]
pub fn Modal(props: &ModalProps) -> Html {
  html!(
    <div class={if props.state {"absolute left-2/4 p-4 text-white rounded-lg shadow-2xl bg-indigo-500 shadow-indigo-500/50"} else {"hidden"}}>
      {"모달입니다"}
    </div>
  )
}