use std::str::FromStr;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub enum TournamentType {
  #[serde(rename="tournament")]
  Tournament,
  #[serde(rename="swissLeague")]
  SwissLeague,
}

impl FromStr for TournamentType {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "tournament" => Ok(TournamentType::Tournament),
      "swiss_league" => Ok(TournamentType::SwissLeague),
      _ => Err(()),
    }
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentState {
  pub id: Uuid,
  #[serde(rename="tournamentType")]
  pub tournament_type: TournamentType,
  pub participants: u8,
  pub title: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TournamentStatus {
  #[serde(rename="prepare")]
  Prepare,
  #[serde(rename="in_progress")]
  InProgress,
  #[serde(rename="done")]
  Done,
}

impl FromStr for TournamentStatus {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "prepare" => Ok(TournamentStatus::Prepare),
      "in_progress" => Ok(TournamentStatus::InProgress),
      "done" => Ok(TournamentStatus::Done),
      _ => Err(()),
    }
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentSummary {
  pub id: Uuid,
  pub title: String,
  pub status: TournamentStatus,
}