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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Copy)]
pub enum MatchEnums {
  #[serde(rename="round_of_256")]
  RoundOf256,
  #[serde(rename="round_of_128")]
  RoundOf128,
  #[serde(rename="round_of_64")]
  RoundOf64,
  #[serde(rename="round_of_32")]
  RoundOf32,
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
pub struct Matches(pub HashMap<MatchEnums, HashMap<u8, Vec<Player>>>);
impl Matches {
  pub fn new(participants: u8) -> Self {
    let match_enum_vec = vec![
      (MatchEnums::Final, 1),
      (MatchEnums::SemiFinal, 2),
      (MatchEnums::QuarterFinal, 4),
      (MatchEnums::RoundOf16, 8),
      (MatchEnums::RoundOf32, 16),
      (MatchEnums::RoundOf64, 32),
      (MatchEnums::RoundOf128, 64),
      (MatchEnums::RoundOf256, 128),
    ];
    let mut m = HashMap::new();
    let mut n = 1_f32;
    let mut cnt = 0;
    while n <= (participants as f32).log2().ceil() {
      let mut value_hash_map = HashMap::new();
      for i in 0..match_enum_vec[cnt].1 {
        value_hash_map.insert(i as u8, vec![
          Player::new(""),
          Player::new(""),
        ]);
      };
      m.insert(match_enum_vec[cnt].0, value_hash_map);
      n += 1_f32;
      cnt += 1;
    };

    Self(m)
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentState {
  pub id: Uuid,
  #[serde(rename="tournamentType")]
  pub tournament_type: TournamentType,
  pub participants: u8,
  pub title: String,
  pub matches: Matches,
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
  pub fn new(name: &str) -> Player {
    Player { name: String::from(name) }
    // if name.len() > 0 {
    //   return Some(Player { name: String::from(name) })
    // } else {
    //   return None
    // }
  }
}