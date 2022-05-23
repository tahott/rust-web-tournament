use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  MainPage,
  #[at("/tournament/:id")]
  TournamentPage { id: String },
}