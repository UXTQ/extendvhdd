
# aHash     ![Build Status](https://img.shields.io/github/actions/workflow/status/tkaitchuck/aHash/rust.yml?branch=master) ![Licence](https://img.shields.io/crates/l/ahash) ![Downloads](https://img.shields.io/crates/d/ahash) 

AHash is the [fastest](https://github.com/tkaitchuck/aHash/blob/master/compare/readme.md#Speed), 
[DOS resistant hash](https://github.com/tkaitchuck/aHash/wiki/How-aHash-is-resists-DOS-attacks) currently available in Rust.
AHash is intended *exclusively* for use in in-memory hashmaps. 

AHash's output is of [high quality](https://github.com/tkaitchuck/aHash/blob/master/compare/readme.md#Quality) but aHash is **not** a cryptographically secure hash.

## Design

Because AHash is a keyed hash, each map will produce completely different hashes, which cannot be predicted without knowing the keys.