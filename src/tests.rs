use super::*;

#[test]
fn keys() {
    assert_eq!(u8::max_bits().cluster_bits(), 4u8);
    assert_eq!(u16::max_bits().cluster_bits(), 8u16);
    assert_eq!(u32::max_bits().cluster_bits(), 16u32);
    assert_eq!(u64::max_bits().cluster_bits(), 32u64);
    assert_eq!(u128::max_bits().cluster_bits(), 64u128);
}

#[test]
fn default_impl() {
    VanEmdeBoasTree::<u32, u32>::default();
    VanEmdeBoasTree::<u64, u64>::default();
    VanEmdeBoasTree::<u128, u128>::default();
}

#[test]
fn is_empty() {
    let mut t = VanEmdeBoasTree::<u32, u32>::new();
    assert_eq!(t.is_empty(), true);
    t.insert(1, 10);
    assert_eq!(t.is_empty(), false);
    t.remove(&1);
    assert_eq!(t.is_empty(), true);
}

#[test]
fn insert_same_key_overwrites() {
    let mut t = VanEmdeBoasTree::<u32, u32>::new();
    assert_eq!(t.insert(1, 10), None);
    assert_eq!(t.successor(&0), Some((1, 10)));
    // Return the old value.
    assert_eq!(t.insert(1, 30), Some(10));
    assert_eq!(t.successor(&0), Some((1, 30)));
}

#[test]
fn insert_get() {
    let mut t = VanEmdeBoasTree::<u32, u32>::new();
    t.insert(1, 10);
    assert_eq!(t.get(&0), None);
    assert_eq!(t.get(&1), Some(10));
    t.insert(3, 30);
    t.insert(4, 40);
    assert_eq!(t.get(&2), None);
    assert_eq!(t.get(&3), Some(30));
    assert_eq!(t.get(&4), Some(40));
    assert_eq!(t.get(&5), None);
}

#[test]
fn insert_successor() {
    let mut t = VanEmdeBoasTree::<u32, u32>::new();
    t.insert(1, 10);
    assert_eq!(t.successor(&0), Some((1, 10)));
    t.insert(3, 30);
    assert_eq!(t.successor(&0), Some((1, 10)));
    assert_eq!(t.successor(&2), Some((3, 30)));
}

#[test]
fn insert_predecessor() {
    let mut t = VanEmdeBoasTree::<u32, u32>::new();
    t.insert(3, 30);
    assert_eq!(t.predecessor(&4), Some((3, 30)));
    t.insert(1, 10);
    assert_eq!(t.predecessor(&4), Some((3, 30)));
    assert_eq!(t.predecessor(&2), Some((1, 10)));
}

#[test]
fn insert_remove_successor() {
    let mut t = VanEmdeBoasTree::<u32, u32>::new();
    t.insert(1, 10);
    t.remove(&1);
    assert_eq!(t.successor(&0), None);
}

#[test]
fn successor_when_not_in_cluster() {
    let mut t = VanEmdeBoasTree::<u32, u32>::new();
    t.insert(1, 10);
    t.insert(u32::MAX, 30);
    assert_eq!(t.successor(&2), Some((u32::MAX, 30)));
}

#[test]
fn predecessor_when_not_in_cluster() {
    let mut t = VanEmdeBoasTree::<u32, u32>::new();
    t.insert(1, 10);
    t.insert(u32::MAX, 30);
    assert_eq!(t.predecessor(&u32::MAX), Some((1, 10)));
}

#[test]
fn remove_after_two_inserts_increasing_order() {
    let mut t = VanEmdeBoasTree::<u32, u32>::new();
    t.insert(0, 0);
    t.insert(1, 1);
    t.remove(&0);
    assert_eq!(t.get(&0), None);
}

#[test]
fn remove_after_two_inserts_decreasing_order() {
    let mut t = VanEmdeBoasTree::<u32, u32>::new();
    t.insert(1, 1);
    t.insert(0, 0);
    t.remove(&1);
    assert_eq!(t.get(&1), None);
}
