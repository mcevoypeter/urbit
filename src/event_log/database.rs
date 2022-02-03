use lmdb::Environment;
use std::{collections::HashMap, path::Path};

pub trait KeyValStore<K, V> {
    fn new(path: &Path) -> Self;

    fn get(&self, keys: Vec<K>) -> HashMap<K, Option<V>>;

    fn put(&self, keys: Vec<K>) -> HashMap<K, Option<V>>;
}

pub struct Lmdb {
    _env: Environment,
}

impl<K, V> KeyValStore<K, V> for Lmdb {
    fn new(path: &Path) -> Self {
        // - Create a directory at [path] if it doesn't already exist.
        // - mdb_env_create(): create an environment
        // - mdb_env_open(): open the just-created environment
        unimplemented!("{}", path.display())
    }

    fn get(&self, _keys: Vec<K>) -> HashMap<K, Option<V>> {
        unimplemented!()
    }

    fn put(&self, _keys: Vec<K>) -> HashMap<K, Option<V>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {}
