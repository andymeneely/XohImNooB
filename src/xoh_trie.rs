use std::{collections::HashMap};

pub struct XohTrie {
  head : TrieLink
}

type TrieLink = Option<Box<TrieNode>>;

struct TrieNode {
  is_word: bool,
  alphabet : HashMap<char, TrieLink>
}

impl TrieNode {
  fn new() -> Self {
    TrieNode {
      is_word: false,
      alphabet : HashMap::new()
    }
  }
}

impl XohTrie {
  pub fn new() -> Self {
    XohTrie { head: None }
  }

  // pub fn insert(&mut self, word : &str){
  //   let new_head = self.insert_recursive(&mut self.head, word);
  //   self.head = new_head;
  // }

  // fn insert_recursive(&mut self, link : &mut TrieLink, word : &str) -> &mut TrieLink {
  //   match word.chars().nth(0) {
  //     Some(_c) => {
  //       &mut None
  //     },
  //     None => { //Empty string? We're done.
  //       match link {
  //         Some(_n) => &mut None,
  //         None => &mut Some(Box::new(TrieNode::new()))
  //       }
  //     }
  //   }
  // }
    // return match word.chars().nth(0) {
    //   Some(c) => {
    //     match self.alphabet.get(&c) {
    //       Some( child ) => { // Letter already exists in node, traverse
    //         child.as_deref().and_then(|&t| t.insert(&word[1..]))
    //       },
    //       None => {
    //         let new_child = Some(Box::new(XohTrie::init()));
    //         self.alphabet.insert(c,new_child);
    //         return self.insert(&word[1..]);
    //       }
    //     }
    //   }
    //   None => { //empty string, so we're a leaf (and a word!)
    //     self.is_word = true;
    //     return None
    //   }
    // }

}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn init_trie(){
    //   let t = XohTrie::new();
    //   assert_eq!(true, t.head.is_none());
    // }

    // fn add_a() {
    //   let t = XohTrie::new();
    //   // t.insert("a");
    //   assert_eq!(false, t.head.unwrap().is_word);
    //   assert_eq!(true, t.head.unwrap().alphabet.get(&'a').unwrap().unwrap().is_word);
    // }

}
