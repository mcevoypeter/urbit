mod snapshot;

use std::{cmp::Ordering, collections::VecDeque, path::Path};

use nock::Cell;

use crate::{
    event_log::snapshot::{Patch, Snapshot},
    kernel::Kernel,
    Error,
};

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

#[allow(dead_code)]
pub struct Event {
    id: u64,
    req: Cell,
}

impl Eq for Event {}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Evt for Event {
    type Id = u64;
    type Request = Cell;

    fn _id(&self) -> &Self::Id {
        &self.id
    }

    fn _request(&self) -> &Self::Request {
        &self.req
    }
}

#[allow(dead_code)]
struct Epoch {
    id: u64,
    patch: Option<Patch>,
    events: VecDeque<Event>,
}

impl Epoch {
    fn _id(&self) -> &u64 {
        &self.id
    }

    fn _first_event(&self) -> Option<&Event> {
        self.events.front()
    }

    fn _last_event(&self) -> Option<&Event> {
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
    epochs: VecDeque<Epoch>,
}

impl EvtLog for EventLog {
    type Event = Event;

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
