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

    #[test]
    fn predecessor_successor_five_keys(
        k1 in any::<u64>(),
        k2 in any::<u64>(),
        k3 in any::<u64>(),
        k4 in any::<u64>(),
        k5 in any::<u64>(),
    ) {
        // There could be duplicates.
        let mut keys = vec![k1, k2, k3, k4, k5];
        verify_predecessor_successor(keys.as_mut_slice())?
    }
}

fn verify_predecessor_successor(keys: &mut [u64]) -> Result<(), TestCaseError> {
    let mut t = VanEmdeBoasTree::<u64, u64>::new();
    for k in keys.iter() {
        t.insert(*k, *k);
    }
    // Sort keys after inserting.
    keys.sort_unstable();

    let min = t.min();
    prop_assert_eq!(min, Some((keys[0], keys[0])));
    let mut key = min.unwrap().0;
    let mut i = 0;
    loop {
        prop_assert!(i < keys.len());
        let successor = t.successor(&key);
        match successor {
            Some((k, _)) => {
                prop_assert!(k > key);
                key = k;
            }
            None => break,
        }
        i += 1;
    }

    let max = t.max();
    prop_assert_eq!(max, Some((*keys.last().unwrap(), *keys.last().unwrap())));
    let mut key = max.unwrap().0;
    let mut i = 0;
    loop {
        prop_assert!(i < keys.len());
        let predecessor = t.predecessor(&key);
        match predecessor {
            Some((k, _)) => {
                prop_assert!(k < key);
                key = k;
            }
            None => break,
        }
        i += 1;
    }

    Ok(())
}
