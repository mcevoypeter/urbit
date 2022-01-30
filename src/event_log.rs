use std::{collections::VecDeque, path::Path};

use nock::Cell;

use crate::{
    snapshot::{SnapshotBase, SnapshotPatch},
    Error, Kernel,
};

#[allow(dead_code)]
struct Event {
    id: u64,
    req: Cell,
}

impl Event {
    fn _id(&self) -> &u64 {
        &self.id
    }

    fn _request(&self) -> &Cell {
        &self.req
    }
}

#[allow(dead_code)]
struct Epoch {
    id: u64,
    patch: Option<SnapshotPatch>,
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
    snapshot: Option<SnapshotBase>,
    epochs: VecDeque<Epoch>,
}

impl EventLog {
    /// Create an event log rooted at the given path.
    fn _new(_path: &Path) -> Result<Self, Error> {
        unimplemented!()
    }

    fn _path(&self) -> &Path {
        &self.path
    }

    fn _first_epoch(&self) -> Option<&Epoch> {
        self.epochs.front()
    }

    fn _last_epoch(&self) -> Option<&Epoch> {
        self.epochs.back()
    }

    fn _append(self, _evt: Event) -> Result<Self, Error> {
        unimplemented!()
    }

    fn _replay(self, _kern: Kernel) -> Result<Kernel, Error> {
        unimplemented!()
    }

    fn _rollover(self) -> Result<Epoch, Error> {
        unimplemented!()
    }

    fn _truncate(self) -> Result<(), Error> {
        unimplemented!()
    }
}
