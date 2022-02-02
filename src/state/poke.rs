use nock::{Cell, Noun};

use crate::{
    event_log::EventLog,
    kernel::Kernel,
    state::{Req, Response, StagedResp},
};

/// Write request.
struct PokeRequest(Cell);

impl Req for PokeRequest {
    type Output = PokeResponse;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn evaluate(self, _arvo: Kernel) -> (Self::Output, Kernel) {
        unimplemented!()
    }
}

/// Uncommitted write response.
struct PokeResponse(Cell, Noun);

impl StagedResp for PokeResponse {
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
