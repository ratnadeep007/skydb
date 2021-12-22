use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash<T>(data: &T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod test {

    #[test]
    fn hasher_working() {
        let hash = super::calculate_hash(&"Test String".to_string());
        assert_eq!(hash, 10247586138395716317);
    }
}
