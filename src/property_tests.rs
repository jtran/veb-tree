use proptest::prelude::*;

use crate::VanEmdeBoasTree;

proptest! {
    #[test]
    fn get_what_was_inserted_one_key(k1 in any::<u64>()) {
        let mut t = VanEmdeBoasTree::<u64, u64>::new();
        t.insert(k1, k1);
        prop_assert_eq!(t.get(&k1), Some(k1));
    }

    #[test]
    fn get_what_was_inserted_two_keys(k1 in any::<u64>(), k2 in any::<u64>()) {
        let mut t = VanEmdeBoasTree::<u64, u64>::new();
        t.insert(k1, k1);
        t.insert(k2, k2);
        prop_assert_eq!(t.get(&k1), Some(k1));
        prop_assert_eq!(t.get(&k2), Some(k2));
    }

    #[test]
    fn get_what_was_inserted_three_keys(k1 in any::<u64>(), k2 in any::<u64>(), k3 in any::<u64>()) {
        let mut t = VanEmdeBoasTree::<u64, u64>::new();
        t.insert(k1, k1);
        t.insert(k2, k2);
        t.insert(k3, k3);
        prop_assert_eq!(t.get(&k1), Some(k1));
        prop_assert_eq!(t.get(&k2), Some(k2));
        prop_assert_eq!(t.get(&k3), Some(k3));
    }

    #[test]
    fn remove_one_key(k1 in any::<u64>()) {
        let mut t = VanEmdeBoasTree::<u64, u64>::new();
        t.insert(k1, k1);
        t.remove(&k1);
        prop_assert_eq!(t.get(&k1), None);
    }

    #[test]
    fn remove_two_keys(k1 in any::<u64>(), k2 in any::<u64>()) {
        let mut t = VanEmdeBoasTree::<u64, u64>::new();
        t.insert(k1, k1);
        t.insert(k2, k2);
        t.remove(&k1);
        prop_assert_eq!(t.get(&k1), None);
        t.remove(&k2);
        prop_assert_eq!(t.get(&k2), None);
    }

    #[test]
    fn remove_three_keys(k1 in any::<u64>(), k2 in any::<u64>(), k3 in any::<u64>()) {
        let mut t = VanEmdeBoasTree::<u64, u64>::new();
        t.insert(k1, k1);
        t.insert(k2, k2);
        t.insert(k3, k3);
        t.remove(&k1);
        prop_assert_eq!(t.get(&k1), None);
        t.remove(&k2);
        prop_assert_eq!(t.get(&k2), None);
        t.remove(&k3);
        prop_assert_eq!(t.get(&k3), None);
    }
}
