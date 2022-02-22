use crate::{error::Error, kernel::Kernel};
use std::path::Path;

trait EvtLog: Sized {
    type Evt: Evt;

    fn _new(_path: &Path) -> Result<Self, Error>;

    fn _path(&self) -> &Path;

    fn append(self, _evt: Self::Evt) -> Result<Self, Self>;

    fn _replay(&self, _kern: Kernel) -> Result<Kernel, Error>;

    fn _truncate(&mut self, _evt: Self::Evt) -> Result<(), Error>;
}

trait Evt: Ord + Sized {
    type Id;
    type Req;

    fn _id(&self) -> &Self::Id;

    fn _request(&self) -> &Self::Req;
}
