use core::fmt;
use std::{fmt::{Formatter, Debug, Display}, error::Error};

use uuid::Uuid;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Request, Response};

use crate::types::{TournamentState, TournamentSummary};

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
  err: JsValue,
}

impl Display for FetchError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    Debug::fmt(&self.err, f)
  }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
  fn from(value: JsValue) -> Self {
    Self { err: value }
  }
}

pub async fn get_tournament_list() -> Result<Vec<TournamentSummary>, FetchError> {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let request = Request::new_with_str_and_init("../products/tournament.json", &opts)?;

  let window = gloo::utils::window();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let resp = resp_value.dyn_into::<Response>().unwrap();

  let json = JsFuture::from(resp.json()?).await?;
  let tournament_list = json.into_serde::<Vec<TournamentSummary>>().unwrap();

  Ok(tournament_list)
}

pub async fn get_tournament_detail(id: Uuid) -> Result<TournamentState, FetchError> {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let request = Request::new_with_str_and_init("../products/tournament.json", &opts)?;

  let window = gloo::utils::window();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let resp = resp_value.dyn_into::<Response>().unwrap();

  let json = JsFuture::from(resp.json()?).await?;
  let tournament_details = json.into_serde::<Vec<TournamentState>>().unwrap();

  let tournament_detail = tournament_details.iter().find(|tournament| tournament.id == id);
  
  let tournament = tournament_detail.unwrap().clone();

  Ok(tournament)
}