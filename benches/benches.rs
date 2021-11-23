#![feature(test)]

extern crate test;

use xohimnoob::xoh_corpus::XohCorpus;
use xohimnoob::{brute_mine_xoh, fast_mine_xoh, mine_xoh, xoh_hash};

use test::Bencher;


#[bench]
fn single_fast_mine_xoh(b: &mut Bencher) {
  let corpus =XohCorpus::new();
  let pw = xoh_hash(corpus.generate_pw().as_str());
  b.iter(|| {
    fast_mine_xoh(pw.as_str(), &corpus)
  })
}

#[bench]
fn bench_new_mine_xoh(b: &mut Bencher){
  let corpus = XohCorpus::new();
  let pw = corpus.generate_pw();
  let pw_str = pw.as_str();
  let hash = xoh_hash(pw_str);
  b.iter(|| {
    mine_xoh(pw_str, &hash, &corpus)
  })
}

#[bench]
fn bench_worst_contains(b: &mut Bencher) {
  let corpus = XohCorpus::new();
  let str = "🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨🅨";
  let mut done = false;
  let all_words = corpus.all_words;
  b.iter(|| {
    for w in all_words.to_owned() {
      done = done && str.contains(w.as_str());
    }
  })
}

#[bench]
fn bench_brute_mine_xoh(b: &mut Bencher) {
  let corpus =XohCorpus::new();
  let pw = corpus.generate_pw();
  let pw_str = pw.as_str();
  let hash = xoh_hash(pw_str);
  b.iter(|| {
    brute_mine_xoh(pw_str, &hash, &corpus)
  })
}

#[bench]
fn fast_is_word(b: &mut Bencher) {
  let corpus =XohCorpus::new();
  let str = xoh_hash(corpus.generate_pw().as_str());
  b.iter(|| {
    corpus.is_word(str.as_str());
  })
}


