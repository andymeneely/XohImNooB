use sha2::{Sha256, Digest};
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;
use std::time::Instant;



pub fn xoh_hash(s: &String) -> String {
    return base64::encode(Sha256::digest(s.as_bytes()));
}

pub fn next_word(word_list : &Vec<String>) -> String {
    let mut rng = thread_rng();
    return word_list.choose(&mut rng).expect("Error getting word").to_string();
}

pub fn load_words(only_large : bool) -> Vec<String> {
    let file = File::open(Path::new("data/xkcd.txt")).expect("Error opening words file");
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

pub struct AwesomeHash {
    decorated_hash : String,
    score: u32
}

impl fmt::Display for AwesomeHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}", self.decorated_hash, self.score)
    }
}

pub fn decorate(hash: &str, word: &str) -> String {
    let decorated_word:String = word.chars().map(|c| match c{
        'a' => '🅐',
       'b' => '🅑',
       'c' => '🅒',
       'd' => '🅓',
       'e' => '🅔',
       'f' => '🅕',
       'g' => '🅖',
       'h' => '🅗',
       'i' => '🅘',
       'j' => '🅙',
       'k' => '🅚',
       'l' => '🅛',
       'm' => '🅜',
       'n' => '🅝',
       'o' => '🅞',
       'p' => '🅟',
       'q' => '🅠',
       'r' => '🅡',
       's' => '🅢',
       't' => '🅣',
       'u' => '🅤',
       'v' => '🅥',
       'w' => '🅦',
       'x' => '🅧',
       'y' => '🅨',
       'z' => '🅩',
       'A' => '🅐',
       'B' => '🅑',
       'C' => '🅒',
       'D' => '🅓',
       'E' => '🅔',
       'F' => '🅕',
       'G' => '🅖',
       'H' => '🅗',
       'I' => '🅘',
       'J' => '🅙',
       'K' => '🅚',
       'L' => '🅛',
       'M' => '🅜',
       'N' => '🅝',
       'O' => '🅞',
       'P' => '🅟',
       'Q' => '🅠',
       'R' => '🅡',
       'S' => '🅢',
       'T' => '🅣',
       'U' => '🅤',
       'V' => '🅥',
       'W' => '🅦',
       'X' => '🅧',
       'Y' => '🅨',
       'Z' => '🅩',
        _ => c
    }).collect();
    return hash.replace(word, decorated_word.as_str());

}

pub fn finish_awesomeness(hash : &String, big_word : &str, all_words : &Vec<String>) -> Vec<AwesomeHash> {
    let mut score = big_word.len() as u32 * big_word.len() as u32;
    let mut awe_list = Vec::new();
    let mut word_list = Vec::new();

    let mut decorated_hash = decorate(hash, big_word);
    let mut done  = false;
    while !done {
        let mut found_word = false;
        for word in all_words { //this isn't exhaustive but whatevs
            if decorated_hash.contains(word) {
                done = false;
                found_word = true;
                decorated_hash = decorate(&decorated_hash, word);
                word_list.push(word);
                score += word.len() as u32 * word.len() as u32;
            }
        }
        if !found_word {
            done = true;
        }
    }
    awe_list.push(AwesomeHash {
        decorated_hash,
        score
    });
    return awe_list;
}

pub fn mine_password(hash : &String, all_words : &Vec<String>, big_words : &Vec<String>) -> Vec<AwesomeHash> {
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

pub fn generate_pw(words : &Vec<String>) -> String {
    let pw = next_word(&words);
    let pw2 = next_word(&words);
    let pw3 = next_word(&words);
    return format!("{} {} {}", pw , pw2, pw3);
}

fn main() {
    let all_words = load_words(false);
    let big_words = load_words(true);
    let before = Instant::now();
    let mut counter = 0;
    loop {
        let pw = generate_pw(&all_words);
        let xoh_hash = xoh_hash(&pw);
        let awe_list = mine_password(&xoh_hash, &all_words, &big_words);
        for awe in awe_list {
            if awe.score > 50 {
                println!("{:24}\t{}\t{}", pw, xoh_hash, awe);
            }
        }
        counter += 1;
        if counter % 10_000 == 0  {
            let elapsed = before.elapsed();
            let speed = counter as f64 / elapsed.as_secs_f64();
            println!("Finished {:10} in time: {:.2?} or {:.1}/s",counter, elapsed, speed)
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
    fn words_load_fine(){
        let mut word_list = load_words(false);
        assert_eq!(3630,word_list.len());
        // load sorted by length
        assert_eq!(String::from("understandings"), word_list.pop().unwrap());
    }

    #[test]
    fn words_no_quotes(){
        let word_list = load_words(false);
        assert_eq!(word_list.contains(&String::from("doesn't")), false);
    }

    #[test]
    fn decorate_simple() {
        let exp = decorate(&String::from("myword"), "word");
        assert_eq!("🅜🅨word", exp);
    }
    #[test]
    fn contains_works() {
        assert_ne!("🅜🅨word", "myword");
    }

}
