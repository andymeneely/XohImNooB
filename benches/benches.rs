#![feature(test)]

extern crate test;

use xohimnoob::xoh_corpus::XohCorpus;
use xohimnoob::{brute_mine_xoh, fast_mine_xoh, mine_xoh, xoh_hash};

use test::Bencher;


#[bench]
fn single_fast_mine_xoh(b: &mut Bencher) {
  let corpus =XohCorpus::new();
  let pw = corpus.generate_pw();
  b.iter(|| {
    fast_mine_xoh(pw.as_str(), &corpus)
  })
}

#[bench]
fn bench_new_mine_xoh(b: &mut Bencher){
  let corpus =XohCorpus::new();
  let pw = corpus.generate_pw();
  let pw_str = pw.as_str();
  let hash = xoh_hash(pw_str);
  b.iter(|| {
    mine_xoh(pw_str, &hash, &corpus)
  })
}

#[bench]
fn bench_brute_mine_xoh(b: &mut Bencher){
  let corpus =XohCorpus::new();
  let pw = corpus.generate_pw();
  let pw_str = pw.as_str();
  let hash = xoh_hash(pw_str);
  b.iter(|| {
    brute_mine_xoh(pw_str, &hash, &corpus)
  })
}
