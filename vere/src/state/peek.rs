use crate::{
    kernel::Kernel,
    state::{Req, Res},
};
use nock::{cell::Cell, noun::Noun};

#[allow(dead_code)]
struct PeekReq {
    req: Cell,
}

#[allow(dead_code)]
struct PeekRes {
    req: Cell,
    res: Noun,
}

impl Req for PeekReq {
    type Res = PeekRes;

    fn evaluate(self, arvo: Kernel) -> (Self::Res, Kernel) {
        let req = Noun::Cell(self.req.clone());
        let (res, arvo) = arvo.evaluate(req);
        (Self::Res { req: self.req, res }, arvo)
    }
}

impl Res for PeekRes {}
