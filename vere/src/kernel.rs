use nock::{Cell, Noun};
use std::path::Path;

pub struct Kernel(Cell);

impl Kernel {
    fn _new(pill: Option<&Path>) -> Self {
        match pill {
            Some(_) => {
                unimplemented!("open from local file")
            }
            None => {
                unimplemented!("download from bootstrap.urbit.org")
            }
        }
    }

    pub fn evaluate(self, req: Noun) -> (Noun, Self) {
        unimplemented!("{}", req)
    }
}
