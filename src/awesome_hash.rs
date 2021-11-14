use std::fmt;

pub struct AwesomeHash {
  pub decorated_hash : String,
  pub score: u32,
}

impl fmt::Display for AwesomeHash {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}\t{}", self.decorated_hash, self.score)
  }
}