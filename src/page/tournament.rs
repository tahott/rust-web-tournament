use yew::prelude::*;

use crate::component::Round;

#[derive(Properties, PartialEq)]
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
pub fn tournament(props: &Props) -> Html {
  let cols = to_round(props.participants, 1, vec![]);
  let round: Vec<Html> = cols.clone().iter().map(|r| {
    html! {
      <Round matches={*r} />
    }

  }).collect();
  html!(
    <div class="container mx-auto bg-sky-500">
      <div class={format!("grid grid-cols-[repeat({},_minmax(0,1fr))] gap-4", cols.len())}>
        {round}
      </div>
    </div>
  )
}