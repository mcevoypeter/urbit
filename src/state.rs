mod peek;
pub mod poke;

use nock::{Cell, Noun};

use crate::{error::Error, event_log::EvtLog, kernel::Kernel};

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

trait Req {
    type Output: StagedResp;

    /// Get the request as a cell.
    fn as_cell(&self) -> &Cell;

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
