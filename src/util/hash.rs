use blake2::{Blake2b, Digest};

pub fn hash_str(src: &String) -> String {
    let mut hasher = Blake2b::new();
    hasher.input(src.as_bytes());
    let hash = hasher.result();
    format!("{:x}", hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_hash() {
        let hash1 = hash_str(&"foo".to_string());
        let hash2 = hash_str(&"foo".to_string());
        let hash3 = hash_str(&"bar".to_string());

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
