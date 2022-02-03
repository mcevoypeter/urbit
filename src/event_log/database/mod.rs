pub mod lmdb;

use std::{collections::HashMap, path::Path};

pub trait KeyValStore<K, V> {
    fn new(path: &Path) -> Self;

    fn get(&self, keys: Vec<K>) -> HashMap<K, Option<V>>;

    fn put(&self, keys: Vec<K>) -> HashMap<K, Option<V>>;
}
