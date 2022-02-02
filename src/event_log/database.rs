use serde::{de::DeserializeOwned, Serialize};

pub trait KeyValStore {
    type Val: DeserializeOwned + Serialize;

    fn read(key: u64) -> Result<Self::Val, ()>;

    fn write(key: u64, val: Self::Val) -> Result<Option<Self::Val>, ()>;
}

pub struct Lmdb;

impl KeyValStore for Lmdb {
    type Val = Vec<u8>;

    fn read(key: u64) -> Result<Self::Val, ()> {
        unimplemented!("{}", key)
    }

    fn write(key: u64, val: Self::Val) -> Result<Option<Self::Val>, ()> {
        unimplemented!("{} {:?}", key, val)
    }
}
