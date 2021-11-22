
use std::time::Instant;

mod found_db;
use found_db::FoundDB;

mod string_decorator;


use xohimnoob::mine_xoh;
use xohimnoob::xoh_hash;
use xohimnoob::xoh_corpus::XohCorpus;


fn main() {
    let corpus = XohCorpus::new();
    let before = Instant::now();
    let mut found = FoundDB::init();
    let mut counter = 0;

    loop {
        let pw = corpus.generate_pw();
        let xoh_hash = xoh_hash(&pw);
        let awe = mine_xoh(pw.as_str(), &xoh_hash, &corpus);
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