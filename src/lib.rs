use nock::{Cell, Noun};

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
    type Next: StagedResponse;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Pass the request to the kernel and generate a response.
    fn evaluate(self, arvo: Kernel) -> (Self::Next, Kernel);
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

    fn evaluate(self, _arvo: Kernel) -> (PeekResponse, Kernel) {
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

    fn evaluate(self, _arvo: Kernel) -> (PokeResponse, Kernel) {
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
