use core::fmt;
use std::{fmt::{Formatter, Debug, Display}, error::Error};

use serde::{Deserialize};
use uuid::Uuid;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Request, Response};

#[derive(Deserialize)]
pub enum TournamentStatus {
  Prepare,
  InProgress,
  Done,
}

#[derive(Deserialize)]
pub struct TournamentSummary {
  id: Uuid,
  title: String,
  status: TournamentStatus,
}

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

  let request = Request::new_with_str_and_init("", &opts)?;

  let window = gloo::utils::window();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let resp = resp_value.dyn_into::<Response>().unwrap();

  let json = JsFuture::from(resp.json()?).await?;
  let tournament_list = json.into_serde::<Vec<TournamentSummary>>().unwrap();

  Ok(tournament_list)
}
