use nock::{Cell, Noun};

use crate::{
    event_log::EventLog,
    kernel::Kernel,
    state::{Req, Response, StagedResp},
};

/// Read request.
struct PeekRequest {
    _req: Cell,
}

impl Req for PeekRequest {
    type Output = PeekResponse;

    fn evaluate(self, _arvo: Kernel) -> (Self::Output, Kernel) {
        unimplemented!()
    }
}

/// Uncommitted read response.
struct PeekResponse {
    _req: Cell,
    _res: Noun,
}

impl StagedResp for PeekResponse {
    type Output = Response;
    type Log = EventLog;

    fn commit(self, _evt_log: Self::Log) -> (Self::Output, Self::Log) {
        unimplemented!()
    }
}
