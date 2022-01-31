use std::path::Path;

use nock::Cell;

/// Arvo kernel.
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
}
