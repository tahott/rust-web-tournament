use std::{str::FromStr, collections::{HashMap}};

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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum MatchEnums {
  #[serde(rename="round_of_16")]
  RoundOf16,
  #[serde(rename="quarter_final")]
  QuarterFinal,
  #[serde(rename="semi_final")]
  SemiFinal,
  #[serde(rename="final")]
  Final,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentState {
  pub id: Uuid,
  #[serde(rename="tournamentType")]
  pub tournament_type: TournamentType,
  pub participants: u8,
  pub title: String,
  pub matches: Option<HashMap<MatchEnums, Option<HashMap<u8, Vec<Player>>>>>,
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

#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Player {
  pub name: String,
}

impl Player {
  pub fn new(name: &str) -> Option<Player> {
    if name.len() > 0 {
      return Some(Player { name: String::from(name) })
    } else {
      return None
    }
  }
}