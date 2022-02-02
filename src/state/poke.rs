use std::cmp::Ordering;

use nock::{Cell, Noun};

use crate::{
    event_log::{database::Lmdb, EventLog, Evt, EvtLog},
    kernel::Kernel,
    state::{Req, Response, StagedResp},
};

/// Write request.
#[derive(Debug)]
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

    fn evaluate(self, arvo: Kernel) -> (Self::Output, Kernel) {
        let req = self.req.clone();
        let (res, arvo) = arvo.evaluate(Noun::Cell(self.req));
        (
            Self::Output {
                req: Self { id: self.id, req },
                res,
            },
            arvo,
        )
    }
}

/// Uncommitted write response.
#[derive(Debug)]
pub struct PokeResponse {
    req: PokeRequest,
    res: Noun,
}

impl StagedResp for PokeResponse {
    type Output = Response;
    type Log = EventLog<Lmdb>;

    fn commit(self, evt_log: Self::Log) -> (Self::Output, Self::Log) {
        match evt_log.append(self.req) {
            Ok(evt_log) => (Response { res: self.res }, evt_log),
            Err(_evt_log) => {
                let _err = todo!();
                #[allow(unreachable_code)]
                (Response { res: _err }, _evt_log)
            }
        }
    }
}
