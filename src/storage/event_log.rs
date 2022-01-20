use crate::Error;
use crate::event::UrEvent;
use crate::storage::EventLog;

/*=================================================================================================
 * Structs
 */

/// The event log data structure.
struct UrEventLog;

/*=================================================================================================
 * Implementations
 */

impl EventLog for UrEventLog {
    type E = UrEvent;

    fn append(self, _evt: Self::E) -> Self {
        self
    }

    fn replay(&self) -> Result<(), Error> {
        Ok(())
    }
}

/*=================================================================================================
 * Tests
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
