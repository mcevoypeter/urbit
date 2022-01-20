use crate::*;
use crate::event::*;

/*=================================================================================================
 * Traits
 */

/// The event log interface.
trait EventLog: Sized {
    type E: Event;

    /// Append an event to the event log.
    fn append(self, evt: Self::E) -> Self;

    /// Replay the event log.
    fn replay(&self) -> Result<(), Error>;
}

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
    use crate::event_log::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
