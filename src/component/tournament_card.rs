use yew::prelude::*;

use crate::types::TournamentStatus;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
  pub title: String,
  pub status: TournamentStatus,
}

#[function_component(TournamentCard)]
pub fn tournament_card(props: &Props) -> Html {
  html! {
    <div class="p-4 rounded hover:border hover:border-solid hover:border-red-300">
      <div class="font-bold text-xl mb-2">{props.title.clone()}</div>
      <div class="px-2 pt-2 pb-2">
        <span class="inline-block bg-gray-200 rounded-full px-2 py-1 text-sm font-semibold text-gray-700 mr-2 mb-2">
          {format!("{:?}", props.status)}
        </span>
      </div>
    </div>
  }
}