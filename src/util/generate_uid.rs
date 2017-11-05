use rand::{OsRng, Rng};
use std::io;

pub fn generate_uid(len: usize) -> io::Result<String> {
    OsRng::new().map(|mut r| r.gen_ascii_chars().take(len).collect())
}
