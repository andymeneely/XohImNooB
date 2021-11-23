use rand::thread_rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeBounds;
use std::path::Path;
use trie_rs::{Trie, TrieBuilder};

pub struct XohCorpus{
  pub all_words : Vec<String>,
  pub big_words : Vec<String>,
  // pub trie : Trie<u8>,
  pub cache : HashSet<Vec<u8>>,
  pub longest : usize
}

impl XohCorpus {
  pub fn new() -> XohCorpus {
      // let mut builder = TrieBuilder::new();
      let all_words = load_words(false);
      let mut longest = 0;
      let mut cache = HashSet::new();
      for word in &all_words {
        // builder.push(word);

        let word_str = word.as_bytes().into();
        cache.insert(word_str);
        if word.len() > longest {
          longest = word.len();
        }
      }
      XohCorpus {
        all_words,
        big_words : load_words(true),
        // trie : builder.build(),
        cache,
        longest
      }
  }

  pub fn is_word(&self, word : &str) -> bool {
    return self.cache.contains(word.as_bytes());
    // self.trie.exact_match(word)
  }

  pub fn generate_pw(&self) -> String {
    let words = &self.all_words;
    let pw = self.next_word(words);
    let pw2 = self.next_word(words);
    let pw3 = self.next_word(words);
    return format!("{} {} {}", pw , pw2, pw3);
  }

  fn next_word(&self, word_list : &Vec<String>) -> String {
    let mut rng = thread_rng();
    return word_list.choose(&mut rng).expect("Error getting word").to_string();
  }
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



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn words_load_fine(){
      let mut word_list = load_words(false);
      assert_eq!(true, word_list.len() > 3000);
      // load sorted by length
      assert_eq!(String::from("telecommunications"), word_list.pop().unwrap());
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

  #[test]
  fn play_with_trie(){
    let mut builder = TrieBuilder::new();
    builder.push("a");
    builder.push("abc");
    let trie = builder.build();
    assert_eq!(true, trie.exact_match("a"));
    assert_eq!(false, trie.exact_match("ab"));
    assert_eq!(true, trie.exact_match("abc"));
  }

}