mod xkcd_words;
use sha2::{Sha256, Digest};

pub fn xoh_hash(s: &String) -> String {
    return base64::encode(Sha256::digest(s.as_bytes()));
}

fn main() {
    xkcd_words::load_words();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_works(){
        assert_eq!(String::from("XohImNooBHFR0OVvjcYpJ3NgPQ1qq73WKhHvch0VQtg="),
                    xoh_hash(&String::from("password")))
    }
}
