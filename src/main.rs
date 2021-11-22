use std::cmp;

use std::time::Instant;
use sha2::{Sha256, Digest};

mod found_db;
use found_db::FoundDB;

mod string_decorator;
use string_decorator::decorate;

mod awesome_hash;
use awesome_hash::AwesomeHash;

mod xoh_corpus;
use xoh_corpus::XohCorpus;
use xoh_corpus::generate_pw;

const SHA_BYTE_LENGTH : usize = 176;

pub fn xoh_hash(s: &String) -> String {
    return base64::encode(Sha256::digest(s.as_bytes()));
}

pub fn fast_mine_xoh(hash : &str, corpus : &XohCorpus) -> Option<String> {
    let n = hash.chars().count();
    for i in 0..n-1 {
        // let end = cmp::min(hash.len() - i, i + corpus.longest + 1);
        
        for j in ((i + 1)..n).rev() {
            // println!("{},{}", i, j);
            let ith_byte_index = hash.char_indices().nth(i).unwrap().0;
            let jth_byte_index = hash.char_indices().nth(j).unwrap().0;
            let sub_str = &hash[ith_byte_index..jth_byte_index];
            if corpus.is_word(sub_str) {
                return Some(String::from(sub_str))
            }
        }
    }
    None
}

pub fn mine_xoh(pw: String, hash : &str, corpus : &XohCorpus) -> AwesomeHash {
    let mut score = 0;
    let mut decorated_hash = hash.to_lowercase();
    let mut done = false;
    // while !done {
        let mined = fast_mine_xoh(&decorated_hash, corpus);
        match mined {
            Some(word) => {
                score += word.len() * word.len();
                let new_decorated_hash = decorate(&decorated_hash.as_str(), word.as_str());
                decorated_hash = new_decorated_hash;
            },
            None => done = true
        }
    // }
    
    AwesomeHash{
        pw,
        decorated_hash: String::from(decorated_hash),
        score : score as u32,
        words: vec![],
     }
    
}

// pub fn mine_xoh(pw: String, hash : &String, corpus : &XohCorpus) -> Option<AwesomeHash> {
//     let all_words = &corpus.all_words;
//     let big_words = &corpus.big_words;
//     let mut decorated_hash = String::from(hash);
//     let hash = hash.to_ascii_lowercase();
//     let mut words = vec![String::from(""); SHA_BYTE_LENGTH];
//     let found = false;
//     let mut score = 0;
//     for word in big_words {
//         if hash.contains(word.as_str()) {
//             let mut done = false;
//             while !done {
//                 let mut found_word = false;
//                 for word in all_words {
//                     if let Some(i) = decorated_hash.find(word) {
//                         done = false;
//                         found_word = true;
//                         decorated_hash = decorate(&decorated_hash, word);
//                         words[i] = format!("{} ", word); //append space
//                         score += word.len() as u32 * word.len() as u32;
//                     }
//                 }
//                 if !found_word {
//                     done = true;
//                 }
//             }
//         }
//     }
//     if found {
//         return Some(AwesomeHash{
//             pw,
//             decorated_hash,
//             score,
//             words,
//         });
//     } else {
//         return None;
//     }
// }

fn main() {
    let corpus = XohCorpus::init();
    let before = Instant::now();
    let mut found = FoundDB::init();
    let mut counter = 0;

    loop {
        let pw = generate_pw(&corpus.all_words);
        let xoh_hash = xoh_hash(&pw);
        let awe = mine_xoh(pw, &xoh_hash, &corpus);
        if awe.score > 48 {
            found.add(awe);
        }
        counter += 1;
        if counter % 10_000 == 0  {
            let elapsed = before.elapsed();
            let speed = counter as f64 / elapsed.as_secs_f64();
            println!("Finished {:10} in time: {:.2?} or {:.1}/s, {}",counter, elapsed, speed, found.report());
            found.save();
        }
    }
}

#[cfg(test)]
mod tests {
    use trie_rs::TrieBuilder;

    use super::*;

    #[test]
    fn hash_works(){
        let str = String::from("XohImNooBHFR0OVvjcYpJ3NgPQ1qq73WKhHvch0VQtg=");
        assert_eq!(str, xoh_hash(&String::from("password")))
    }

    #[test]
    fn string_index_confusion(){
        let str = "🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨";
        assert_eq!(44, str.chars().count());
        assert_eq!(SHA_BYTE_LENGTH, str.as_bytes().len());
    }

    #[test]
    fn fast_mine_xoh_easy(){
        let mut corpus = XohCorpus::init();
        let mut builder = TrieBuilder::new();
        builder.push("ab");
        builder.push("abc");
        builder.push("abcd");
        let trie = builder.build();
        corpus.trie = trie;
        let hash = "---abc-----";
        let result = fast_mine_xoh( &String::from(hash), &corpus);
        assert_eq!("abc", result.unwrap());
    }


    #[test]
    fn fast_mine_xoh_already_decorated(){
        let mut corpus = XohCorpus::init();
        let mut builder = TrieBuilder::new();
        builder.push("ab");
        builder.push("abc");
        builder.push("abcd");
        let trie = builder.build();
        corpus.trie = trie;
        let hash = "---🅨abc🅨---";
        let result = fast_mine_xoh( &String::from(hash), &corpus);
        assert_eq!("abc", result.unwrap());
    }

}
