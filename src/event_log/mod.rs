pub mod database;
mod snapshot;

use std::{collections::VecDeque, fmt, path::Path};

use crate::{
    error::Error,
    event_log::{
        database::{KeyValStore, Lmdb},
        snapshot::{Patch, Snapshot},
    },
    kernel::Kernel,
    state::poke::PokeRequest,
};

//=================================================================================================
// Traits
//=================================================================================================

/// Generic event log interface.
pub trait EvtLog: Sized {
    type Evt;

    fn _new(_path: &Path) -> Result<Self, Error>;

    fn _path(&self) -> &Path;

    fn append(self, _evt: Self::Evt) -> Result<Self, Self>;

    fn _replay(&self, _kern: Kernel) -> Result<Kernel, Error>;

    fn _truncate(&mut self, _evt: Self::Evt) -> Result<(), Error>;
}

/// Generic event interface.
pub trait Evt: Ord + Sized {
    type Id;
    type Req;

    fn _id(&self) -> &Self::Id;

    fn _request(&self) -> &Self::Req;
}

///  +-------------------+
///  | Snapshot          |
///  +-------------------+
///            |
///  +-------------------+
///  | Epoch             |
///  | +---------------+ |
///  | | Event         | |
///  | +---------------+ |
///  | +---------------+ |
///  | | Event         | |
///  | +---------------+ |
///  | +---------------+ |
///  | | Event         | |
///  | +---------------+ |
///  | +===============+ |
///  | | Patch         | |
///  | +===============+ |
///  +--------------------+
///            |
///  +-------------------+
///  | Epoch             |
///  | +---------------+ |
///  | | Event         | |
///  | +---------------+ |
///  | +---------------+ |
///  | | Event         | |
///  | +---------------+ |
///  | +---------------+ |
///  | | Event         | |
///  | +---------------+ |
///  | +===============+ |
///  | | Patch         | |
///  | +===============+ |
///  +--------------------+

//=================================================================================================
// Structs
//=================================================================================================

#[allow(dead_code)]
struct Epoch<E: Evt> {
    id: u64,
    patch: Option<Patch>,
    events: VecDeque<E>,
}

impl Epoch<PokeRequest> {
    fn _id(&self) -> &u64 {
        &self.id
    }

    fn _first_event(&self) -> Option<&PokeRequest> {
        self.events.front()
    }

    fn _last_event(&self) -> Option<&PokeRequest> {
        self.events.back()
    }

    fn _drop(self) -> Result<(), Error> {
        unimplemented!()
    }
}

#[allow(dead_code)]
pub struct EventLog<T: KeyValStore> {
    path: Box<Path>,
    snapshot: Option<Snapshot>,
    epochs: VecDeque<Epoch<PokeRequest>>,
    db: T,
}

impl fmt::Debug for EventLog<Lmdb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventLog")
            .field("path", &self.path)
            .finish()
    }
}

impl EvtLog for EventLog<Lmdb> {
    type Evt = PokeRequest;

    /// Create an event log rooted at the given path.
    fn _new(_path: &Path) -> Result<Self, Error> {
        unimplemented!()
    }

    fn _path(&self) -> &Path {
        &self.path
    }

    fn append(self, evt: Self::Evt) -> Result<Self, Self> {
        unimplemented!("{:?}", evt)
    }

    fn _replay(&self, _kern: Kernel) -> Result<Kernel, Error> {
        unimplemented!()
    }

    fn _truncate(&mut self, _evt: Self::Evt) -> Result<(), Error> {
        unimplemented!()
    }
}

//=================================================================================================
// Tests
//=================================================================================================

#[cfg(test)]
mod tests {}
