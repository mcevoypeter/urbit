use nock::{Cell, Noun};

use crate::{
    error::Error,
    event_log::{EventLog, EvtLog},
    kernel::Kernel,
};

/// Trait-based flow:
///
/// +--------------+ evaluate +--------------+   commit   +---------------+
/// | Req          | -------> | StagedResp   | ---------> | CommittedResp |
/// +--------------+          +--------------+            +---------------+
///
///
/// Struct-based flow:
///
/// +--------------+ evaluate +--------------+
/// | PeekRequest  | -------> | PeekResponse | \
/// +--------------+          +--------------+  \
///                                              \ commit +----------+
///                                               ------> | Response |
///                                              /        +----------+
///                                             /
/// +--------------+ evaluate +--------------+ /
/// | PokeRequest  | -------> | PokeResponse |
/// +--------------+          +--------------+

//=================================================================================================
// Traits
//=================================================================================================

trait Req {
    type Output: StagedResp;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Pass the request to the kernel, generating a staged response and a new kernel.
    fn evaluate(self, arvo: Kernel) -> (Self::Output, Kernel);
}

trait StagedResp {
    type Output: CommittedResp;
    type Log: EvtLog;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Get the response as a noun.
    fn response(&self) -> &Noun;

    /// Commit the response to the event log, generating a committed response and a new event log.
    fn commit(self, evt_log: Self::Log) -> (Self::Output, Self::Log);
}

trait CommittedResp {
    fn send(self) -> Result<(), Error>;
}

//=================================================================================================
// Structs
//=================================================================================================

/// Read request.
struct PeekRequest(Cell);

impl Req for PeekRequest {
    type Output = PeekResponse;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn evaluate(self, _arvo: Kernel) -> (PeekResponse, Kernel) {
        unimplemented!()
    }
}

/// Write request.
struct PokeRequest(Cell);

impl Req for PokeRequest {
    type Output = PokeResponse;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn evaluate(self, _arvo: Kernel) -> (PokeResponse, Kernel) {
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

/// Committed (read or write) response.
struct Response(Noun);

impl CommittedResp for Response {
    fn send(self) -> Result<(), Error> {
        unimplemented!()
    }
}

//=================================================================================================
// Tests
//=================================================================================================

#[cfg(test)]
mod tests {}
