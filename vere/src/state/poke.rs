use crate::{
    kernel::Kernel,
    state::{Req, Res},
};
use nock::{Cell, Noun};

#[allow(dead_code)]
struct PokeReq {
    req: Cell,
}

#[allow(dead_code)]
struct PokeRes {
    req: Cell,
    res: Noun,
}

impl Req for PokeReq {
    type Res = PokeRes;

    fn evaluate(self, arvo: Kernel) -> (Self::Res, Kernel) {
        let req = Noun::Cell(self.req.clone());
        let (res, arvo) = arvo.evaluate(req);
        (Self::Res { req: self.req, res }, arvo)
    }
}

impl Res for PokeRes {}
