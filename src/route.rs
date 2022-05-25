use uuid::Uuid;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  MainPage,
  #[at("/tournament/:id")]
  TournamentPage { id: Uuid },
  #[not_found]
  #[at("/404")]
  NotFound,
}