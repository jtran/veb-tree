//! This implements a cache-oblivious tree with successor and predecessor
//! operations.
//!
//! u is the size of the key universe.  n is the number of items in the tree,
//! each in the range [0, u - 1].
//!
//! Operations run in O(log(log(u))) time.  The data structure uses O(n *
//! log(log(u))) space.
//!
//! To achieve the space bound, we need to use a sparse data-structure, so keys
//! must be hashable.
//!
//! The tree accepts any Clone type for values, but you probably want to use
//! values that are Copy.
//!
//! Internally, each cluster should be of size square root of u.  The number of
//! clusters should be square root of u.  The size of the summary should be
//! square root of u.
//!
//! For example, if u = 2^32, then the cluster size is 2^16.
use core::hash::Hash;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::mem::swap;

#[derive(Debug, Clone)]
pub struct VanEmdeBoasTree<K, V> {
    min: Option<(K, V)>,
    max: Option<(K, V)>,
    summary: Option<Box<VanEmdeBoasTree<K, ()>>>,
    clusters: HashMap<K, VanEmdeBoasTree<K, V>>,
    cluster_size: K,
    #[cfg(any(test, feature = "safety_checks"))]
    max_key: K,
}

impl<K, V> VanEmdeBoasTree<K, V>
where
    K: VanEmdeBoasKey,
{
    pub fn new(max_key: K) -> VanEmdeBoasTree<K, V> {
        VanEmdeBoasTree {
            min: None,
            max: None,
            summary: None,
            clusters: HashMap::new(),
            cluster_size: max_key.cluster_size(),
            #[cfg(any(test, feature = "safety_checks"))]
            max_key,
        }
    }

    /// Returns true if the tree has no elements.
    pub fn is_empty(&self) -> bool {
        self.min.is_none()
    }
}

impl<K, V> VanEmdeBoasTree<K, V>
where
    K: VanEmdeBoasKey + Ord + Clone + Hash + Eq + Debug,
    V: Clone + Debug,
{
    /// Get the maximum element in the tree.  Runs in O(1) time.
    pub fn max(&self) -> Option<(K, V)> {
        self.max.clone()
    }

    /// Get the minimum element in the tree.  Runs in O(1) time.
    pub fn min(&self) -> Option<(K, V)> {
        self.min.clone()
    }

    /// Insert a key-value pair into the tree.  Runs in O(lg lg u) time.
    pub fn insert(&mut self, mut key: K, mut value: V) {
        #[cfg(any(test, feature = "safety_checks"))]
        assert!(key <= self.max_key);

        if self.is_empty() {
            // When currently empty, be lazy to prevent recursive calls.
            self.min = Some((key.clone(), value.clone()));
            self.max = Some((key, value));
            return;
        }

        // If it's less than the min, swap it with the min.
        if let Some((min_key, min_value)) = self.min.as_mut() {
            if key < *min_key {
                swap(min_key, &mut key);
                swap(min_value, &mut value);
            }
        }
        // If it's greater than the max, swap it with the max.
        if let Some((max_key, max_value)) = self.max.as_mut() {
            if key > *max_key {
                swap(max_key, &mut key);
                swap(max_value, &mut value);
            }
        }

        let h = key.high(self.cluster_size.clone());
        let cluster = self
            .clusters
            .entry(h.clone())
            .or_insert_with(|| VanEmdeBoasTree::new(self.cluster_size.clone()));
        // Only recurse on the summary if the cluster is empty and is about to
        // transition to non-empty.  This prevents unneeded recursive calls on
        // the summary.
        if cluster.is_empty() {
            self.summary
                .get_or_insert_with(|| Box::new(VanEmdeBoasTree::new(self.cluster_size.clone())))
                .insert(h, ());
        }
        // When cluster is empty, this recursive call will trigger the lazy case
        // and run in constant time.
        let l = key.low(self.cluster_size.clone());
        cluster.insert(l, value);
    }

    /// Remove a key from the tree.  Runs in O(lg lg u) time.
    pub fn remove(&mut self, key: &K) {
        let mut key = Cow::Borrowed(key);
        if let Some((min_key, _)) = self.min.as_ref() {
            if *key == *min_key {
                match self.summary.as_ref().and_then(|summary| summary.min()) {
                    None => {
                        self.min = None;
                        self.max = None;
                        return;
                    }
                    Some((summary_min, _)) => {
                        let cluster = self
                            .clusters
                            .get_mut(&summary_min)
                            .expect("cluster for summary min should exist");
                        let (cluster_min, new_min_value) = cluster
                            .min()
                            .expect("cluster for summary min should have a min element");
                        let new_min_key = summary_min.index(cluster_min, self.cluster_size.clone());
                        self.min = Some((new_min_key.clone(), new_min_value));
                        key = Cow::Owned(new_min_key);
                    }
                }
            }
        }

        let h = key.high(self.cluster_size.clone());
        if let Some(cluster) = self.clusters.get_mut(&h) {
            cluster.remove(&key.low(self.cluster_size.clone()));
            if cluster.is_empty() {
                if let Some(summary) = self.summary.as_mut() {
                    summary.remove(&h);
                }
            }
        }

        if let Some((max_key, _)) = self.max.as_ref() {
            if *key == *max_key {
                // TODO: summary should never be None here.
                match self.summary.as_ref().and_then(|summary| summary.max()) {
                    None => {
                        self.max = self.min.clone();
                    }
                    Some((summary_max, _)) => {
                        let cluster = self
                            .clusters
                            .get_mut(&summary_max)
                            .expect("cluster for summary min should exist");
                        let (cluster_max, new_max_value) = cluster
                            .max()
                            .expect("cluster for summary min should have a min element");
                        let new_max_key = summary_max.index(cluster_max, self.cluster_size.clone());
                        self.max = Some((new_max_key, new_max_value));
                    }
                }
            }
        }
    }

    /// Get the successor of the given key.  Runs in O(lg lg u) time.
    pub fn successor(&self, key: &K) -> Option<(K, V)> {
        // If the key is less than the min, then the successor is the min.
        if let Some((min_key, min_value)) = self.min.as_ref() {
            if *key < *min_key {
                return Some((min_key.clone(), min_value.clone()));
            }
        }

        // If the key is less than its cluster's max, then the successor is in
        // that cluster.
        let h = key.high(self.cluster_size.clone());
        if let Some(cluster) = self.clusters.get(&h) {
            if let Some((cluster_max, _)) = cluster.max() {
                let l = key.low(self.cluster_size.clone());
                if l < cluster_max {
                    // Recurse.
                    let successor = cluster.successor(&l);
                    match successor {
                        // This should never happen since we checked that the
                        // key is less than the cluster max.
                        None => panic!("key is less than cluster max, but successor wasn't found; key={key:?}, h={h:?}, l={l:?}, cluster_max={cluster_max:?}"),
                        Some((next_l, v)) => {
                            return Some((h.index(next_l, self.cluster_size.clone()), v));
                        }
                    }
                }
            }
        }

        // Recurse on the summary table to find the next cluster.  The successor
        // is the min in that cluster.
        if let Some(summary) = &self.summary {
            // Recurse.
            if let Some((next_h, _)) = summary.successor(&h) {
                if let Some(next_cluster) = self.clusters.get(&next_h) {
                    if let Some((next_l, v)) = next_cluster.min() {
                        return Some((next_h.index(next_l, self.cluster_size.clone()), v));
                    }
                }
            }
        }

        // If the key is less than the max, then the successor is the max.
        if let Some((max_key, max_value)) = self.max.as_ref() {
            if *key < *max_key {
                return Some((max_key.clone(), max_value.clone()));
            }
        }

        None
    }

    /// Get the predecessor of the given key.  Runs in O(lg lg u) time.
    pub fn predecessor(&self, key: &K) -> Option<(K, V)> {
        // If the key is greater than the max, then the predecessor is the max.
        if let Some((max_key, max_value)) = self.max.as_ref() {
            if *key > *max_key {
                return Some((max_key.clone(), max_value.clone()));
            }
        }

        // If the key is greater than its cluster's min, then the predecessor is
        // in that cluster.
        let h = key.high(self.cluster_size.clone());
        if let Some(cluster) = self.clusters.get(&h) {
            if let Some((cluster_min, _)) = cluster.min() {
                let l = key.low(self.cluster_size.clone());
                if l > cluster_min {
                    // Recurse.
                    let predecessor = cluster.predecessor(&l);
                    match predecessor {
                        // This should never happen since we checked that the
                        // key is less than the cluster min.
                        None => panic!("key is less than cluster min, but predecessor wasn't found; key={key:?}, h={h:?}, l={l:?}, cluster_min={cluster_min:?}"),
                        Some((next_l, v)) => {
                            return Some((h.index(next_l, self.cluster_size.clone()), v));
                        }
                    }
                }
            }
        }

        // Recurse on the summary table to find the previous cluster.  The
        // predecessor is the max in that cluster.
        if let Some(summary) = &self.summary {
            // Recurse.
            if let Some((prev_h, _)) = summary.predecessor(&h) {
                if let Some(prev_cluster) = self.clusters.get(&prev_h) {
                    if let Some((prev_l, v)) = prev_cluster.max() {
                        return Some((prev_h.index(prev_l, self.cluster_size.clone()), v));
                    }
                }
            }
        }

        // If the key is greater than the min, then the predecessor is the min.
        if let Some((min_key, min_value)) = self.min.as_ref() {
            if *key > *min_key {
                return Some((min_key.clone(), min_value.clone()));
            }
        }

        None
    }
}

pub trait VanEmdeBoasKey {
    /// The size of a single cluster, i.e. square root of the size of the
    /// universe.
    fn cluster_size(&self) -> Self;
    /// The cluster number from the key.
    fn high(&self, cluster_size: Self) -> Self;
    /// The index within the cluster from the key.
    fn low(&self, cluster_size: Self) -> Self;
    /// The key from the cluster number and the index within the cluster.
    fn index(&self, low: Self, cluster_size: Self) -> Self;
}

macro_rules! impl_van_emde_boas_key {
    ($typ: ty) => {
        impl VanEmdeBoasKey for $typ {
            fn cluster_size(&self) -> Self {
                (*self as f64).sqrt().floor() as Self
            }

            fn high(&self, cluster_size: Self) -> Self {
                self / cluster_size
            }

            fn low(&self, cluster_size: Self) -> Self {
                self % cluster_size
            }

            fn index(&self, low: Self, cluster_size: Self) -> Self {
                self * cluster_size + low
            }
        }
    };
}

impl_van_emde_boas_key!(u32);
impl_van_emde_boas_key!(u64);
impl_van_emde_boas_key!(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let mut t = VanEmdeBoasTree::<u32, u32>::new(u32::MAX);
        assert_eq!(t.is_empty(), true);
        t.insert(1, 10);
        assert_eq!(t.is_empty(), false);
        t.remove(&1);
        assert_eq!(t.is_empty(), true);
    }

    #[test]
    fn insert_successor() {
        let mut t = VanEmdeBoasTree::<u32, u32>::new(u32::MAX);
        t.insert(1, 10);
        assert_eq!(t.successor(&0), Some((1, 10)));
        t.insert(3, 30);
        assert_eq!(t.successor(&0), Some((1, 10)));
        assert_eq!(t.successor(&2), Some((3, 30)));
    }

    #[test]
    fn insert_predecessor() {
        let mut t = VanEmdeBoasTree::<u32, u32>::new(u32::MAX);
        t.insert(3, 30);
        assert_eq!(t.predecessor(&4), Some((3, 30)));
        t.insert(1, 10);
        assert_eq!(t.predecessor(&4), Some((3, 30)));
        assert_eq!(t.predecessor(&2), Some((1, 10)));
    }

    #[test]
    fn insert_remove_successor() {
        let mut t = VanEmdeBoasTree::<u32, u32>::new(u32::MAX);
        t.insert(1, 10);
        t.remove(&1);
        assert_eq!(t.successor(&0), None);
    }

    #[test]
    fn successor_when_not_in_cluster() {
        let mut t = VanEmdeBoasTree::<u32, u32>::new(u32::MAX);
        t.insert(1, 10);
        t.insert(u32::MAX, 30);
        assert_eq!(t.successor(&2), Some((u32::MAX, 30)));
    }

    #[test]
    fn predecessor_when_not_in_cluster() {
        let mut t = VanEmdeBoasTree::<u32, u32>::new(u32::MAX);
        t.insert(1, 10);
        t.insert(u32::MAX, 30);
        assert_eq!(t.predecessor(&u32::MAX), Some((1, 10)));
    }
}
