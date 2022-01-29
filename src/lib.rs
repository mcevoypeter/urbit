use nock::{Cell, Noun};

/// Trait-based flow:
///
/// +--------------+ fulfill +----------------+ commit +-------------------+
/// | Request      | ------> | StagedResponse | -----> | CommittedResponse |
/// +--------------+         +----------------+        +-------------------+
///
///
/// Struct-based flow:
///
/// +--------------+ fulfill +--------------+
/// | PeekRequest  | ------> | PeekResponse | \
/// +--------------+         +--------------+  \
///                                             \ commit +----------+
///                                              ------> | Response |
///                                             /        +----------+
///                                            /
/// +--------------+ fulfill +--------------+ /
/// | PokeRequest  | ------> | PokeResponse |
/// +--------------+         +--------------+

trait Request {
    type Next: StagedResponse;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Pass the request to the kernel and generate a response.
    fn fulfill(self) -> Self::Next;
}

trait StagedResponse {
    type Next: CommittedResponse;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Get the response as a noun.
    fn response(&self) -> &Noun;

    /// Commit the response to the event log.
    fn commit(self) -> Self::Next;
}

trait CommittedResponse {
    fn send(self) -> Result<(), Error>;
}

/// Arvo kernel.
#[allow(dead_code)]
struct Kernel(Cell);

/// Read request.
#[allow(dead_code)]
struct PeekRequest(Cell);

impl Request for PeekRequest {
    type Next = PeekResponse;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn fulfill(self) -> Self::Next {
        unimplemented!()
    }
}

/// Write request.
#[allow(dead_code)]
struct PokeRequest(Cell);

impl Request for PokeRequest {
    type Next = PokeResponse;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn fulfill(self) -> Self::Next {
        unimplemented!()
    }
}

/// Uncommitted read response.
#[allow(dead_code)]
struct PeekResponse(Cell, Noun);

impl StagedResponse for PeekResponse {
    type Next = Response;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn response(&self) -> &Noun {
        &self.1
    }

    fn commit(self) -> Self::Next {
        unimplemented!()
    }
}

/// Uncommitted write response.
#[allow(dead_code)]
struct PokeResponse(Cell, Noun);

impl StagedResponse for PokeResponse {
    type Next = Response;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn response(&self) -> &Noun {
        &self.1
    }

    fn commit(self) -> Self::Next {
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
struct Error {
    #[allow(dead_code)]
    msg: String,
}
