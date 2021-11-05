use crate::lattices::base_lattices::{MapLattice, Lattice};
use std::hash::Hash;
use crate::lattices::lww_lattice::LWWLattice;
use std::collections::HashMap;

pub struct KvStore {
    pub db: MapLattice<Vec<u8>, LWWLattice<Vec<u8>>, Vec<u8>>
}

impl KvStore {
    #[inline]
    pub fn new() -> KvStore {
        KvStore {
            db: MapLattice {
                element: HashMap::new(),
                __phantom: Default::default()
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.db.get(key)
    }

    pub fn put(&mut self, key: &K, val: &V) {
        self.db.insert(key, val);
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.db.remove(key)
    }
}
