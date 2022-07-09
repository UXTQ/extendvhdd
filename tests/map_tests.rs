#![cfg_attr(feature = "specialize", feature(build_hasher_simple_hash_one))]

use std::hash::{BuildHasher, Hash, Hasher};

use ahash::RandomState;
use criterion::*;
use fxhash::FxHasher;

fn gen_word_pairs() -> Vec<String> {
    let words: Vec<_> = r#"
a, ability, able, about, above, accept, according, account, across, act, action,
activity, actually, add, address, administration, admit, adult, affect, after,
again, against, age, agency, agent, ago, agree, agreement, ahead, air, all,
allow, almost, alone, along, already, also, although, always, American, among,
amount, analysis, and, animal, another, answer, any, anyone, anything, appear,
apply, approach, area, argue, arm, around, arrive, art, article, artist, as,
ask, assume, at, attack, attention, attorney, audience, author, authori