use nock::{Cell, Noun};

use crate::{
    event_log::EventLog,
    kernel::Kernel,
    state::{Req, Response, StagedResp},
};

/// Read request.
struct PeekRequest(Cell);

impl Req for PeekRequest {
    type Output = PeekResponse;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn evaluate(self, _arvo: Kernel) -> (Self::Output, Kernel) {
        unimplemented!()
    }
}

/// Uncommitted read response.
struct PeekResponse(Cell, Noun);

impl StagedResp for PeekResponse {
    type Output = Response;
    type Log = EventLog;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn response(&self) -> &Noun {
        &self.1
    }

    fn commit(self, _evt_log: Self::Log) -> (Self::Output, Self::Log) {
        unimplemented!()
    }
}
