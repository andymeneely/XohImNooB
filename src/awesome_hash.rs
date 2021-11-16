use serde::{Serialize, Deserialize};
use std::cmp::Ordering;

use std::fmt;

#[derive(Serialize, Deserialize, PartialOrd, PartialEq, Eq)]
pub struct AwesomeHash {
  pub decorated_hash : String,
  pub score: u32,
  pub words : Vec<String>,
}

impl fmt::Display for AwesomeHash {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}\t{}", self.decorated_hash, self.score)
  }
}

impl Ord for AwesomeHash {
  fn cmp(&self, other: &Self) -> Ordering {
    self.score.cmp(&other.score)
  }
}