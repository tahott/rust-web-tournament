use yew::prelude::*;
use yew_router::prelude::*;

use crate::{route::{Route}, page::{Main, Tournament}};

fn switch(routes: &Route) -> Html {
  match routes {
    Route::MainPage => html! { <Main /> },
    Route::TournamentPage { id: _ } => html! { <Tournament /> },
  }
}

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <BrowserRouter>
      <Switch<Route> render={Switch::render(switch)} />
    </BrowserRouter>
  }
}
