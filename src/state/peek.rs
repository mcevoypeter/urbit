use nock::{Cell, Noun};

use crate::{
    event_log::EventLog,
    kernel::Kernel,
    state::{Req, Response, StagedResp},
};

/// Read request.
#[derive(Debug)]
struct PeekRequest {
    req: Cell,
}

impl Req for PeekRequest {
    type Output = PeekResponse;

    fn evaluate(self, arvo: Kernel) -> (Self::Output, Kernel) {
        let (res, arvo) = arvo.evaluate(Noun::Cell(self.req));
        (Self::Output { res }, arvo)
    }
}

/// Uncommitted read response.
#[derive(Debug)]
struct PeekResponse {
    res: Noun,
}

impl StagedResp for PeekResponse {
    type Output = Response;
    type Log = EventLog;

    fn commit(self, evt_log: Self::Log) -> (Self::Output, Self::Log) {
        (Response { res: self.res }, evt_log)
    }
}
