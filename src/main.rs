use sha2::{Sha256, Digest};
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn xoh_hash(s: &String) -> String {
    return base64::encode(Sha256::digest(s.as_bytes()));
}

pub fn next_word(word_list : &Vec<String>) -> String {
    let mut rng = thread_rng();
    return word_list.choose(&mut rng).expect("Error getting word").to_string();
}

pub fn load_words() -> Vec<String> {
    let file = File::open(Path::new("data/xkcd.txt")).expect("Error opening words file");
    let mut v = Vec::new();
    for line in io::BufReader::new(file).lines() {
      let clean = line.expect("Reading error").replace("'","");
      v.push(clean);
    }
    return v;
}

pub fn mine_password(hash : &String, words : &Vec<String>) -> (usize, Vec<String>) {
    let mut score = 0;
    let mut found = Vec::new();
    let hash = hash.to_ascii_lowercase();
    for word in words {
        if hash.contains(word.as_str()) {
            score += word.len() * word.len();
            found.push(word.to_string());
        }
    }
    return (score, found);
}

pub fn generate_pw(words : &Vec<String>) -> String {
    let pw = next_word(&words);
    let pw2 = next_word(&words);
    let pw3 = next_word(&words);
    return format!("{} {} {}", pw , pw2, pw3);
}

fn main() {
    let words = load_words();
    loop {
        let pw = generate_pw(&words);
        let xoh_hash = xoh_hash(&pw);
        let (score, found) = mine_password(&xoh_hash, &words);
        if score > 80 {
            println!("{:24}\t{:}\t{}\t{}", pw, score, xoh_hash, found.join(" "));
        }
        // if score > 60 {
        //     break;
        // }
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
      let mut word_list = load_words();
      assert_eq!(3632,word_list.len());
      assert_eq!(String::from("an"), word_list.pop().unwrap());
    }

    #[test]
    fn words_no_quotes(){
        let word_list = load_words();
        assert_eq!(word_list.contains(&String::from("doesn't")), false);
    }

}
