#[allow(dead_code)]
use nock::{Cell, Noun};

use crate::event_log::*;

mod event_log;

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

trait Request {
    type Output: StagedResponse;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Pass the request to the kernel, generating a staged response and a new kernel.
    fn evaluate(self, arvo: Kernel) -> (Self::Output, Kernel);
}

trait StagedResponse {
    type Output: CommittedResponse;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Get the response as a noun.
    fn response(&self) -> &Noun;

    /// Commit the response to the event log, generating a committed response and a new event log.
    fn commit(self, evt_log: EventLog) -> (Self::Output, EventLog);
}

trait CommittedResponse {
    fn send(self) -> Result<(), Error>;
}

/// Arvo kernel.
struct Kernel(Cell);

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

    fn request(&self) -> &Cell {
        &self.0
    }

    fn response(&self) -> &Noun {
        &self.1
    }

    fn commit(self, _evt_log: EventLog) -> (Response, EventLog) {
        unimplemented!()
    }
}

/// Uncommitted write response.
struct PokeResponse(Cell, Noun);

impl StagedResponse for PokeResponse {
    type Output = Response;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn response(&self) -> &Noun {
        &self.1
    }

    fn commit(self, _evt_log: EventLog) -> (Response, EventLog) {
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

/// A Vere-specific error encapsulating an informative error message.
#[derive(Debug)]
enum Error {}
