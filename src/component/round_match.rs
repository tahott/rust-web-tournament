use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub round_match: u8
}

#[function_component(RoundMatch)]
pub fn round_match(props: &Props) -> Html {
 html!(
  html! {
    <div>
      {format!("match {}", props.round_match)}
    </div>
  }
 )
}