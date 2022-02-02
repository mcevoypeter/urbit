use std::cmp::Ordering;

use nock::{Cell, Noun};

use crate::{
    event_log::{EventLog, Evt},
    kernel::Kernel,
    state::{Req, Response, StagedResp},
};

/// Write request.
pub struct PokeRequest {
    id: u64,
    req: Cell,
}

impl Eq for PokeRequest {}

impl Evt for PokeRequest {
    type Id = u64;
    type Request = Cell;

    fn _id(&self) -> &Self::Id {
        &self.id
    }

    fn _request(&self) -> &Self::Request {
        &self.req
    }
}

impl Ord for PokeRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialEq for PokeRequest {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for PokeRequest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Req for PokeRequest {
    type Output = PokeResponse;

    fn evaluate(self, _arvo: Kernel) -> (Self::Output, Kernel) {
        unimplemented!()
    }
}

/// Uncommitted write response.
pub struct PokeResponse {
    _req: Cell,
    _res: Noun,
}

impl StagedResp for PokeResponse {
    type Output = Response;
    type Log = EventLog;

    fn commit(self, _evt_log: Self::Log) -> (Self::Output, Self::Log) {
        unimplemented!()
    }
}
