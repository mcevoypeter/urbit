use nock::{Cell, Noun};

use crate::{
    event_log::{EventLog, EvtLog},
    kernel::Kernel,
    Error,
};

/// Trait-based flow:
///
/// +--------------+ evaluate +----------------+ commit +-------------------+
/// | Request      | -------> | StagedResponse | -----> | CommittedResponse |
/// +--------------+          +----------------+        +-------------------+
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

trait Request {
    type Output: StagedResponse;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Pass the request to the kernel, generating a staged response and a new kernel.
    fn evaluate(self, arvo: Kernel) -> (Self::Output, Kernel);
}

trait StagedResponse {
    type Output: CommittedResponse;
    type Log: EvtLog;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Get the response as a noun.
    fn response(&self) -> &Noun;

    /// Commit the response to the event log, generating a committed response and a new event log.
    fn commit(self, evt_log: Self::Log) -> (Self::Output, Self::Log);
}

trait CommittedResponse {
    fn send(self) -> Result<(), Error>;
}

//=================================================================================================
// Structs
//=================================================================================================

/// Read request.
struct PeekRequest(Cell);

impl Request for PeekRequest {
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

impl Request for PokeRequest {
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

impl StagedResponse for PeekResponse {
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

impl StagedResponse for PokeResponse {
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

impl CommittedResponse for Response {
    fn send(self) -> Result<(), Error> {
        unimplemented!()
    }
}
