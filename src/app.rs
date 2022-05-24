use yew::prelude::*;
use yew_router::prelude::*;

use crate::{route::{Route}, page::{Main, Tournament}};

fn switch(routes: &Route) -> Html {
  match routes {
    Route::MainPage => html! { <Main /> },
    Route::TournamentPage { id: _ } => html! { <Tournament /> },
    Route::NotFound => html! { <div class="container mx-auto py-2">{"Not Found!!"}</div> },
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
