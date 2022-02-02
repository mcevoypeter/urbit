use std::path::Path;

use lmdb::Environment;

pub trait KeyValStore<K, V> {
    fn new(path: &Path) -> Self;

    fn read(&self, key: K) -> Result<V, ()>;

    fn write(&self, key: K, val: V) -> Result<Option<V>, ()>;
}

pub struct Lmdb {
    _env: Environment,
}

impl<K, V> KeyValStore<K, V> for Lmdb {
    fn new(path: &Path) -> Self {
        unimplemented!("{}", path.display())
    }

    fn read(&self, _key: K) -> Result<V, ()> {
        unimplemented!()
    }

    fn write(&self, _key: K, _val: V) -> Result<Option<V>, ()> {
        unimplemented!()
    }
}
