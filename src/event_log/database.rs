pub trait KeyValStore {
    fn read(key: u64) -> Result<Vec<u8>, ()>;

    fn write(key: u64, val: Vec<u8>) -> Result<Option<Vec<u8>>, ()>;
}

pub struct Lmdb;

impl KeyValStore for Lmdb {
    fn read(key: u64) -> Result<Vec<u8>, ()> {
        unimplemented!("{}", key)
    }

    fn write(key: u64, val: Vec<u8>) -> Result<Option<Vec<u8>>, ()> {
        unimplemented!("{} {:?}", key, val)
    }
}
