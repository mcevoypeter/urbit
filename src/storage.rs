use crate::Error;
use crate::event::Event;

mod event_log;
mod snapshot;

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

/// The snapshot interface.
trait Snapshot {}
