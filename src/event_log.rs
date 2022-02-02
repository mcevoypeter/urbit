mod snapshot;

use std::{collections::VecDeque, path::Path};

use crate::{
    error::Error,
    event_log::snapshot::{Patch, Snapshot},
    kernel::Kernel,
    state::poke::PokeRequest,
};

//=================================================================================================
// Traits
//=================================================================================================

/// Generic event log interface.
pub trait EvtLog: Sized {
    type Event;

    fn _new(_path: &Path) -> Result<Self, Error>;

    fn _path(&self) -> &Path;

    fn _append(&mut self, _evt: Self::Event) -> Result<(), Error>;

    fn _replay(&self, _kern: Kernel) -> Result<Kernel, Error>;

    fn _truncate(&mut self, _evt: Self::Event) -> Result<(), Error>;
}

/// Generic event interface.
pub trait Evt: Ord + Sized {
    type Id;
    type Request;

    fn _id(&self) -> &Self::Id;

    fn _request(&self) -> &Self::Request;
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
pub struct EventLog {
    path: Box<Path>,
    snapshot: Option<Snapshot>,
    epochs: VecDeque<Epoch<PokeRequest>>,
}

impl EvtLog for EventLog {
    type Event = PokeRequest;

    /// Create an event log rooted at the given path.
    fn _new(_path: &Path) -> Result<Self, Error> {
        unimplemented!()
    }

    fn _path(&self) -> &Path {
        &self.path
    }

    fn _append(&mut self, _evt: Self::Event) -> Result<(), Error> {
        unimplemented!()
    }

    fn _replay(&self, _kern: Kernel) -> Result<Kernel, Error> {
        unimplemented!()
    }

    fn _truncate(&mut self, _evt: Self::Event) -> Result<(), Error> {
        unimplemented!()
    }
}

//=================================================================================================
// Tests
//=================================================================================================

#[cfg(test)]
mod tests {}
