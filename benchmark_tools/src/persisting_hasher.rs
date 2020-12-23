use std::convert::TryInto;
use std::fs::File;
use std::hash::{Hasher, BuildHasher};
use std::io::{BufWriter, Write};
use std::sy