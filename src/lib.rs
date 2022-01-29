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

    /// Get the kernel that will be used to generate the response.
    fn kernel(&self) -> &Kernel;

    /// Pass the request to the kernel and generate a response.
    fn fulfill(self) -> Self::Next;
}

trait StagedResponse {
    type Next: CommittedResponse;

    /// Get the request as a noun.
    fn request(&self) -> &Cell;

    /// Get the response as a noun.
    fn response(&self) -> &Noun;

    /// Get the kernel that resulted from generating the response.
    fn kernel(&self) -> &Kernel;

    /// Commit the response to the event log.
    fn commit(self) -> Self::Next;
}

trait CommittedResponse {
    fn send(self) -> Result<(), Error>;
}

struct Kernel(Cell);

/// Read request.
#[allow(dead_code)]
struct PeekRequest {
    req: Cell,
    kern: Kernel,
}

impl Request for PeekRequest {
    type Next = PeekResponse;

    fn request(&self) -> &Cell {
        &self.req
    }

    fn kernel(&self) -> &Kernel {
        &self.kern
    }

    fn fulfill(self) -> Self::Next {
        PeekResponse {
            req: self.req,
            res: Noun::new_atom(0),
            kern: self.kern,
        }
    }
}

/// Write request.
#[allow(dead_code)]
struct PokeRequest {
    req: Cell,
    kern: Kernel,
}

impl Request for PokeRequest {
    type Next = PokeResponse;

    fn request(&self) -> &Cell {
        &self.req
    }

    fn kernel(&self) -> &Kernel {
        &self.kern
    }

    fn fulfill(self) -> Self::Next {
        PokeResponse {
            req: self.req,
            res: Noun::new_atom(0),
            kern: self.kern,
        }
    }
}

/// Uncommitted read response.
#[allow(dead_code)]
struct PeekResponse {
    req: Cell,
    res: Noun,
    kern: Kernel,
}

impl StagedResponse for PeekResponse {
    type Next = Response;

    fn request(&self) -> &Cell {
        &self.req
    }

    fn response(&self) -> &Noun {
        &self.res
    }

    fn kernel(&self) -> &Kernel {
        &self.kern
    }

    fn commit(self) -> Self::Next {
        Response(self.res)
    }
}

/// Uncommitted write response.
#[allow(dead_code)]
struct PokeResponse {
    req: Cell,
    res: Noun,
    kern: Kernel,
}

impl StagedResponse for PokeResponse {
    type Next = Response;

    fn request(&self) -> &Cell {
        &self.req
    }

    fn response(&self) -> &Noun {
        &self.res
    }

    fn kernel(&self) -> &Kernel {
        &self.kern
    }

    fn commit(self) -> Self::Next {
        Response(self.res)
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
