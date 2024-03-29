use super::*;

#[test]
fn key_cluster_size() {
    assert_eq!(u8::cluster_size(&u8::max_size()), 4u8);
    assert_eq!(u16::cluster_size(&u16::max_size()), 8u8);
    assert_eq!(u32::cluster_size(&u32::max_size()), 16u8);
    assert_eq!(u64::cluster_size(&u64::max_size()), 32u8);
    assert_eq!(u128::cluster_size(&u128::max_size()), 64u8);
}

#[test]
fn default_impl() {
    VebTreeMap::<u32, u32>::default();
    VebTreeMap::<u64, u64>::default();
    VebTreeMap::<u128, u128>::default();
}

#[test]
fn is_empty() {
    let mut t = VebTreeMap::<u32, u32>::new();
    assert!(t.is_empty());
    t.insert(1, 10);
    assert!(!t.is_empty());
    t.remove(&1);
    assert!(t.is_empty());
}

#[test]
fn clear() {
    let mut t = VebTreeMap::<u32, u32>::new();
    t.insert(1, 10);
    t.insert(2, 20);
    t.insert(3, 30);
    assert!(!t.is_empty());
    t.clear();
    assert!(t.is_empty());
}

#[test]
fn insert_same_key_overwrites() {
    let mut t = VebTreeMap::<u32, u32>::new();
    assert_eq!(t.insert(1, 10), None);
    assert_eq!(t.successor(&0), Some((1, 10)));
    // Return the old value.
    assert_eq!(t.insert(1, 30), Some(10));
    assert_eq!(t.successor(&0), Some((1, 30)));
}

#[test]
fn insert_get() {
    let mut t = VebTreeMap::<u32, u32>::new();
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
    let mut t = VebTreeMap::<u32, u32>::new();
    t.insert(1, 10);
    assert_eq!(t.successor(&0), Some((1, 10)));
    t.insert(3, 30);
    assert_eq!(t.successor(&0), Some((1, 10)));
    assert_eq!(t.successor(&2), Some((3, 30)));
}

#[test]
fn insert_predecessor() {
    let mut t = VebTreeMap::<u32, u32>::new();
    t.insert(3, 30);
    assert_eq!(t.predecessor(&4), Some((3, 30)));
    t.insert(1, 10);
    assert_eq!(t.predecessor(&4), Some((3, 30)));
    assert_eq!(t.predecessor(&2), Some((1, 10)));
}

#[test]
fn insert_remove_successor() {
    let mut t = VebTreeMap::<u32, u32>::new();
    t.insert(1, 10);
    t.remove(&1);
    assert_eq!(t.successor(&0), None);
}

#[test]
fn successor_when_not_in_cluster() {
    let mut t = VebTreeMap::<u32, u32>::new();
    t.insert(1, 10);
    t.insert(u32::MAX, 30);
    assert_eq!(t.successor(&2), Some((u32::MAX, 30)));
}

#[test]
fn predecessor_when_not_in_cluster() {
    let mut t = VebTreeMap::<u32, u32>::new();
    t.insert(1, 10);
    t.insert(u32::MAX, 30);
    assert_eq!(t.predecessor(&u32::MAX), Some((1, 10)));
}

#[test]
fn remove_after_two_inserts_increasing_order() {
    let mut t = VebTreeMap::<u32, u32>::new();
    t.insert(0, 0);
    t.insert(1, 1);
    t.remove(&0);
    assert_eq!(t.get(&0), None);
}

#[test]
fn remove_after_two_inserts_decreasing_order() {
    let mut t = VebTreeMap::<u32, u32>::new();
    t.insert(1, 1);
    t.insert(0, 0);
    t.remove(&1);
    assert_eq!(t.get(&1), None);
}
