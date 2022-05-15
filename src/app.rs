use yew::prelude::*;

use crate::page::Tournament;

#[function_component(App)]
pub fn app() -> Html {
  html!(
    <div class="py-2">
      <Tournament participants=16 />
    </div>
  )
}