use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;

use crate::awesome_hash::AwesomeHash;


#[derive(Serialize, Deserialize)]
pub struct FoundDB {
  db : HashMap<String, AwesomeHash>
}

impl FoundDB {

  pub fn init() -> FoundDB{
    let found_yml = fs::read_to_string("data/found.yml").expect("Could not read file data/found.yml");
    return serde_yaml::from_str(found_yml.as_str()).expect("Could not parse data/found.yml");
    // if we want to reset
    // return FoundDB{
    //   db : HashMap::new()
    // }
  }

  pub fn add(&mut self, pw: &str, awe : AwesomeHash) {
    self.db.insert(String::from(pw), awe);
  }

  pub fn save(&mut self){
    let found_yml = fs::read_to_string("data/found.yml").expect("Could not read file data/found.yml");
    let saved_db : FoundDB = serde_yaml::from_str(found_yml.as_str()).expect("Could not parse data/found.yml");
    self.db.extend(saved_db.db);
    let new_found_yml = serde_yaml::to_string(self).expect("Could not emit YML for db");
    fs::write("data/found.yml", new_found_yml).expect("Unable to write file");
 }

  pub fn report(&self) -> String {
    let len = self.db.len();
    let max = match self.db.values().max() {
      Some(s) => s.score,
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
        decorated_hash: String::from("foo")
      });
      found_db.save();
    }


}
