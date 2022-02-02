use std::path::Path;

use lmdb::Environment;

pub trait KeyValStore<T> {
    fn new(path: &Path) -> Self;

    fn read(&self, key: u64) -> Result<T, ()>;

    fn write(&self, key: u64, val: T) -> Result<Option<T>, ()>;
}

pub struct Lmdb {
    _env: Environment,
}

impl<T> KeyValStore<T> for Lmdb {
    fn new(path: &Path) -> Self {
        unimplemented!("{}", path.display())
    }

    fn read(&self, key: u64) -> Result<T, ()> {
        unimplemented!("{}", key)
    }

    fn write(&self, key: u64, _val: T) -> Result<Option<T>, ()> {
        unimplemented!("{}", key)
    }
}
