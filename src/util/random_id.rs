use rand::{OsRng, Rng};

/// Generate secure random id whose length is `len`.
pub fn generate_random_id(len: usize) -> String {
    OsRng::new()
        .map(|mut r| r.gen_ascii_chars().take(len).collect())
        .unwrap()
}
