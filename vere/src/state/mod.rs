mod peek;
mod poke;

use crate::{error::Error, kernel::Kernel};

trait Req: Sized {
    type Res: Res;

    fn evaluate(self, arvo: Kernel) -> (Self::Res, Kernel);
}

trait Res: Sized {
    fn send(self) -> Result<(), Error> {
        unimplemented!()
    }
}
