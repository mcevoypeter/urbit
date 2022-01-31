use std::path::Path;

use crate::Error;

#[allow(dead_code)]
pub struct Snapshot {
    path: Box<Path>,
}

impl Snapshot {
    fn _new(_path: &Path) -> Result<Self, Error> {
        unimplemented!()
    }

    /// Apply a patch to the snapshot, creating a new snapshot.
    fn _apply_patch(self, _patch: Patch) -> Result<Self, Error> {
        unimplemented!()
    }

    /// Restore the snapshot to memory.
    fn _restore(self) -> Result<(), Error> {
        unimplemented!()
    }
}

pub struct Patch;

#[cfg(test)]
mod tests {}
