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

pub fn finish_awesomeness(hash : &String, big_word : &str, all_words : &Vec<String>) -> Vec<AwesomeHash> {
    let mut score = big_word.len() as u32 * big_word.len() as u32;
    let mut awe_list = Vec::new();
    let mut words = vec![String::from(""); SHA_BYTE_LENGTH]; //

    let mut decorated_hash = decorate(hash, big_word);
    let mut done  = false;
    while !done {
        let mut found_word = false;
        for word in all_words { //this isn't exhaustive but whatevs
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
    awe_list.push(AwesomeHash {
        decorated_hash,
        score,
        words,
    });
    return awe_list;
}

pub fn mine_password(hash : &String, corpus : &XohCorpus) -> Vec<AwesomeHash> {
    let all_words = &corpus.all_words;
    let big_words = &corpus.big_words;
    let hash = hash.to_ascii_lowercase();
    let mut awe_list = Vec::new();
    for word in big_words {
        if hash.contains(word.as_str()) {
            let mut finished_awe_list = finish_awesomeness(&hash, word, all_words);
            awe_list.append(&mut finished_awe_list);
        }
    }
    return awe_list;
}

fn main() {
    let corpus = XohCorpus::init();
    let before = Instant::now();
    let mut found = FoundDB::init();
    let mut counter = 0;

    loop {
        let pw = generate_pw(&corpus.all_words);
        let xoh_hash = xoh_hash(&pw);
        let awe_list = mine_password(&xoh_hash, &corpus);
        for awe in awe_list {
            if awe.score > 48 {
                found.add(&pw, awe);
            }
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

}
