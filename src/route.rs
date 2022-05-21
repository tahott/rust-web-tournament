use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Copy)]
pub enum Route {
  #[at("/")]
  MainPage,
}