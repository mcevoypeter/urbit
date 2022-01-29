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

trait Kernel<I, O>: Sized
where
    I: Request,
    O: StagedResponse,
{
    /// Get the kernel as a cell.
    fn kernel(&self) -> &Cell;

    /// Evaluate a request, producing a new kernel and the result of the request.
    fn evaluate(self, req: I) -> (Self, Result<O, Error>);
}

trait Request {
    type Out: StagedResponse;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Pass the request to the kernel and generate a response.
    fn fulfill(self) -> Self::Out;
}

trait StagedResponse {
    type Out: CommittedResponse;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Get the response as a noun.
    fn response(&self) -> &Noun;

    /// Commit the response to the event log.
    fn commit(self) -> Self::Out;
}

trait CommittedResponse {
    fn send(self) -> Result<(), Error>;
}

/// Arvo kernel.
struct Arvo(Cell);

/// Kernel interface for peeks.
impl Kernel<PeekRequest, PeekResponse> for Arvo {
    fn kernel(&self) -> &Cell {
        &self.0
    }

    fn evaluate(self, req: PeekRequest) -> (Self, Result<PeekResponse, Error>) {
        unimplemented!()
    }
}

/// Kernel interface for pokes.
impl Kernel<PokeRequest, PokeResponse> for Arvo {
    fn kernel(&self) -> &Cell {
        &self.0
    }

    fn evaluate(self, req: PokeRequest) -> (Self, Result<PokeResponse, Error>) {
        unimplemented!()
    }
}

/// Read request.
#[allow(dead_code)]
struct PeekRequest(Cell);

impl Request for PeekRequest {
    type Out = PeekResponse;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn fulfill(self) -> PeekResponse {
        unimplemented!()
    }
}

/// Write request.
#[allow(dead_code)]
struct PokeRequest(Cell);

impl Request for PokeRequest {
    type Out = PokeResponse;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn fulfill(self) -> PokeResponse {
        unimplemented!()
    }
}

/// Uncommitted read response.
#[allow(dead_code)]
struct PeekResponse(Cell, Noun);

impl StagedResponse for PeekResponse {
    type Out = Response;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn response(&self) -> &Noun {
        &self.1
    }

    fn commit(self) -> Response {
        unimplemented!()
    }
}

/// Uncommitted write response.
#[allow(dead_code)]
struct PokeResponse(Cell, Noun);

impl StagedResponse for PokeResponse {
    type Out = Response;

    fn request(&self) -> &Cell {
        &self.0
    }

    fn response(&self) -> &Noun {
        &self.1
    }

    fn commit(self) -> Response {
        unimplemented!()
    }
}

/// Committed (read or write) response.
struct Response(Noun);

impl CommittedResponse for Response {
    fn send(self) -> Result<(), Error> {
        Ok(())
    }
}

/// A Vere-specific error encapsulating an informative error message.
#[derive(Debug)]
struct Error {
    #[allow(dead_code)]
    msg: String,
}
