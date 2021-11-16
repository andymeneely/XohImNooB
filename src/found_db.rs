use serde::{Serialize, Deserialize};
use std::{cmp::Ordering, fs};

use crate::awesome_hash::AwesomeHash;

#[derive(Serialize, Deserialize)]
pub struct FoundDB {
  db : Vec<FoundDBEntry>
}

#[derive(Serialize, Deserialize, PartialOrd, PartialEq, Eq)]
struct FoundDBEntry {
  pw : String,
  decorated_hash: String,
  score : u32,
  words : String
}

impl Ord for FoundDBEntry {
  fn cmp(&self, other: &Self) -> Ordering {
    return self.score.cmp(&other.score)
  }
}

impl FoundDB {

  pub fn init() -> FoundDB {
    let found_yml = match fs::read_to_string("data/found.yml") {
      Ok(str) => str,
      Err(_e) => {
        let init_str = "---\ndb: []";
        fs::write("data/found.yml", init_str).expect("Could not init data/found.yml");
        String::from(init_str)
      }
    };
    return serde_yaml::from_str(found_yml.as_str()).expect("Could not parse data/found.yml");
   }

  pub fn add(&mut self, pw: &str, awe : AwesomeHash) {
    let e  = FoundDBEntry {
      pw:String::from(pw),
      decorated_hash: awe.decorated_hash.to_string(),
      score: awe.score,
      words: awe.words.join("")
    };
    self.db.push(e);
  }

  pub fn save(&mut self){
    let new_found_yml = serde_yaml::to_string(self).expect("Could not emit YML for db");
    fs::write("data/found.yml", new_found_yml).expect("Unable to write file");
 }

  pub fn report(&self) -> String {
    let len = self.db.len();
    let max = match self.db.iter().max() {
      Some(e) => e.score,
      None => 0
    };
   let output = format!("{} found, best score: {}", len, max);
   return output;
 }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_saved(){
      let mut found_db = FoundDB::init();
      found_db.add("foo", AwesomeHash{
        score: 2,
        decorated_hash: String::from("foo"),
        words: vec![]
      });
      found_db.save();
    }


}
