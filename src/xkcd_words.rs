use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load_words() -> Vec<String> {
  let file = File::open(Path::new("data/xkcd.txt")).expect("Error opening words file");
  let mut v = Vec::new();
  for line in io::BufReader::new(file).lines() {
    v.push(line.expect("Reading error"));
  }
  return v;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_load_fine(){
      let mut word_list = load_words();
      assert_eq!(3632,word_list.len());
      assert_eq!(String::from("an"), word_list.pop().unwrap());
    }
}