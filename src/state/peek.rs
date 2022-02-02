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

    fn as_cell(&self) -> &Cell {
        &self.req
    }

    fn evaluate(self, _arvo: Kernel) -> (Self::Output, Kernel) {
        unimplemented!()
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

    fn request(&self) -> &Cell {
        &self.req
    }

    fn as_noun(&self) -> &Noun {
        &self.res
    }

    fn commit(self, _evt_log: Self::Log) -> (Self::Output, Self::Log) {
        unimplemented!()
    }
}
