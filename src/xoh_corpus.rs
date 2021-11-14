use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct XohCorpus{
  pub all_words : Vec<String>,
  pub big_words : Vec<String>,
}

impl XohCorpus {
  pub fn init() -> XohCorpus {
      XohCorpus {
        all_words : load_words(false),
        big_words : load_words(true),
      }
  }
}

pub fn generate_pw(words : &Vec<String>) -> String {
  let pw = next_word(&words);
  let pw2 = next_word(&words);
  let pw3 = next_word(&words);
  return format!("{} {} {}", pw , pw2, pw3);
}

fn load_words(only_large : bool) -> Vec<String> {
  let file = File::open(Path::new("data/xohswords.txt")).expect("Error opening words file");
  let mut v = Vec::new();
  for line in io::BufReader::new(file).lines() {
    let clean = line.expect("Reading error").replace("'","");
    if clean.len() > 1 {
      if only_large {
          if clean.len() > 3 {
              v.push(clean);
          }
      } else {
          v.push(clean);
      }
    }
  }
  v.sort_by(|a,b| a.len().cmp(&b.len()));
  return v;
}

fn next_word(word_list : &Vec<String>) -> String {
  let mut rng = thread_rng();
  return word_list.choose(&mut rng).expect("Error getting word").to_string();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn words_load_fine(){
      let mut word_list = load_words(false);
      assert_eq!(3630,word_list.len());
      // load sorted by length
      assert_eq!(String::from("understandings"), word_list.pop().unwrap());
  }

  // #[test]
  // fn part_of_speech(){
  //     let pos_model = POSModel::new(Default::default()).unwrap();
  //     let input = ["Oh I'm Noob"];
  //     let output = pos_model.predict(&input);
  //     println!("{:?}", output);
  // }

  #[test]
  fn words_no_quotes(){
      let word_list = load_words(false);
      assert_eq!(word_list.contains(&String::from("doesn't")), false);
  }
}