use std::collections::HashMap;

use std::cmp;
use sha2::{Digest, Sha256};

pub mod xoh_corpus;
use xoh_corpus::XohCorpus;

pub mod awesome_hash;
use awesome_hash::AwesomeHash;

mod string_decorator;
use string_decorator::decorate;

const SHA_BYTE_LENGTH: usize = 176;

pub fn xoh_hash(s: &str) -> String {
    return base64::encode(Sha256::digest(s.as_bytes()));
}

pub fn fast_mine_xoh(hash: &str, corpus: &XohCorpus) -> Option<String> {
    let n = hash.chars().count();
    let mut char2byte_index : HashMap<usize, usize> = HashMap::new();
    let mut i_char = 0;
    for (byte_i, _c) in hash.char_indices() {
        char2byte_index.insert(i_char, byte_i);
        i_char += 1;
    }
    for i in 0..n - 1 { //i,j iterating over chars, not bytes
        let end = cmp::min(hash.len() - i, i + corpus.longest + 1);
        for j in ((i + 1)..end).rev() {
            let ith_byte_index = char2byte_index.get(&i).unwrap().to_owned();
            let jth_byte_index = char2byte_index.get(&j).unwrap().to_owned();
            let sub_str = &hash[ith_byte_index..jth_byte_index];
            if corpus.is_word(sub_str) {
                return Some(String::from(sub_str));
            }
        }
    }
    None
}

pub fn mine_xoh(pw: &str, hash: &str, corpus: &XohCorpus) -> AwesomeHash {
    let mut score = 0;
    let mut decorated_hash = hash.to_lowercase();
    let mut done = false;
    while !done {
        let mined = fast_mine_xoh(&decorated_hash, corpus);
        match mined {
            Some(word) => {
                score += word.len() * word.len();
                let new_decorated_hash = decorate(&decorated_hash.as_str(), word.as_str());
                decorated_hash = new_decorated_hash;
            }
            None => done = true,
        }
    }

    AwesomeHash {
        pw: String::from(pw),
        decorated_hash: String::from(decorated_hash),
        score: score as u32,
        words: vec![],
    }
}

pub fn brute_mine_xoh(pw: &str, hash: &String, corpus: &XohCorpus) -> Option<AwesomeHash> {
    let all_words = &corpus.all_words;
    let big_words = &corpus.big_words;
    let mut decorated_hash = String::from(hash);
    let hash = hash.to_ascii_lowercase();
    let mut words = vec![String::from(""); SHA_BYTE_LENGTH];
    let found = false;
    let mut score = 0;
    for word in big_words {
        if hash.contains(word.as_str()) {
            let mut done = false;
            while !done {
                let mut found_word = false;
                for word in all_words {
                    if let Some(i) = decorated_hash.find(word) {
                        done = false;
                        found_word = true;
                        decorated_hash = decorate(&decorated_hash, word);
                        words[i] = format!("{} ", word); //append space
                        score += word.len() as u32 * word.len() as u32;
                    }
                }
                if !found_word {
                    done = true;
                }
            }
        }
    }
    if found {
        return Some(AwesomeHash {
            pw: String::from(pw),
            decorated_hash,
            score,
            words,
        });
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trie_rs::TrieBuilder;

    #[test]
    fn hash_works() {
        let str = String::from("XohImNooBHFR0OVvjcYpJ3NgPQ1qq73WKhHvch0VQtg=");
        assert_eq!(str, xoh_hash(&String::from("password")))
    }

    #[test]
    fn string_index_confusion() {
        let str = "🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨";
        assert_eq!(44, str.chars().count());
        assert_eq!(SHA_BYTE_LENGTH, str.as_bytes().len());
    }

    #[test]
    fn fast_mine_xoh_easy() {
        let mut corpus = XohCorpus::new();
        let mut builder = TrieBuilder::new();
        builder.push("ab");
        builder.push("abc");
        builder.push("abcd");
        let trie = builder.build();
        corpus.trie = trie;
        let hash = "---abc-----";
        let result = fast_mine_xoh(&String::from(hash), &corpus);
        assert_eq!("abc", result.unwrap());
    }

    #[test]
    fn fast_mine_xoh_already_decorated() {
        let mut corpus = XohCorpus::new();
        let mut builder = TrieBuilder::new();
        builder.push("ab");
        builder.push("abc");
        builder.push("abcd");
        let trie = builder.build();
        corpus.trie = trie;
        let hash = "---🅨abc🅨---";
        let result = fast_mine_xoh(&String::from(hash), &corpus);
        assert_eq!("abc", result.unwrap());
    }
}
