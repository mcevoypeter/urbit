use nock::{Cell, Noun};

use crate::{
    event_log::EventLog,
    kernel::Kernel,
    state::{Req, Response, StagedResp},
};

/// Read request.
struct PeekRequest {
    req: Cell,
}

impl Req for PeekRequest {
    type Output = PeekResponse;

    fn evaluate(self, arvo: Kernel) -> (Self::Output, Kernel) {
        let req = self.req.clone();
        let (res, arvo) = arvo.evaluate(Noun::Cell(self.req));
        (Self::Output { req, res }, arvo)
    }
}

/// Uncommitted read response.
struct PeekResponse {
    req: Cell,
    res: Noun,
}

impl StagedResp for PeekResponse {
    type Output = Response;
    type Log = EventLog;

    fn commit(self, evt_log: Self::Log) -> (Self::Output, Self::Log) {
        unimplemented!("{:?} {:?} {:?}", self.req, self.res, evt_log)
    }
}
