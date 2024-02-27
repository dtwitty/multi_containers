#[cfg(test)]
mod tests {
    use crate::{HashMultiSet, MultiMapBuilder};
    #[test]
    fn library() {
        let mut map = MultiMapBuilder::hash_keys().hash_values().build();
        map.insert(
            "James S.A. Corey".to_string(),
            "Leviathan Wakes".to_string(),
        );
        map.insert("James S.A. Corey".to_string(), "Caliban's War".to_string());
        map.insert("James S.A. Corey".to_string(), "Abaddon's Gate".to_string());
        map.insert("James S.A. Corey".to_string(), "Cibola Burn".to_string());
        map.insert("James S.A. Corey".to_string(), "Nemesis Games".to_string());
        map.insert(
            "James S.A. Corey".to_string(),
            "Babylon's Ashes".to_string(),
        );
        map.insert(
            "James S.A. Corey".to_string(),
            "Persepolis Rising".to_string(),
        );
        map.insert("James S.A. Corey".to_string(), "Tiamat's Wrath".to_string());
        map.insert(
            "James S.A. Corey".to_string(),
            "Leviathan Falls".to_string(),
        );
        map.insert("Isaac Asimov".to_string(), "Foundation".to_string());
        map.insert(
            "Isaac Asimov".to_string(),
            "Foundation and Empire".to_string(),
        );
        map.insert("Isaac Asimov".to_string(), "Second Foundation".to_string());
        map.insert("Isaac Asimov".to_string(), "Foundation's Edge".to_string());
        map.insert(
            "Isaac Asimov".to_string(),
            "Foundation and Earth".to_string(),
        );
        map.insert(
            "Isaac Asimov".to_string(),
            "Prelude to Foundation".to_string(),
        );
        map.insert(
            "Isaac Asimov".to_string(),
            "Forward the Foundation".to_string(),
        );
        map.insert("Dan Simmons".to_string(), "Hyperion".to_string());
        map.insert(
            "Dan Simmons".to_string(),
            "The Fall of Hyperion".to_string(),
        );
        map.insert("Dan Simmons".to_string(), "Endymion".to_string());
        map.insert(
            "Dan Simmons".to_string(),
            "The Rise of Endymion".to_string(),
        );

        assert_eq!(map.num_mappings(), 20);
        assert_eq!(map.get("James S.A. Corey").unwrap().len(), 9);
        assert_eq!(map.get("Isaac Asimov").unwrap().len(), 7);
        assert_eq!(map.get("Dan Simmons").unwrap().len(), 4);
    }

    #[test]
    fn counter() {
        let set = "the quick brown fox jumps over the lazy dog"
            .chars()
            .collect::<HashMultiSet<_, _>>();

        assert_eq!(set.len(), 43);
        assert_eq!(set.count(&'e'), 3);
        assert_eq!(set.count(&'o'), 4);
        assert_eq!(set.count(&'z'), 1);
    }
}
