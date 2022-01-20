use crate::*;

/*=================================================================================================
 * Traits
 */

/// The event interface.
pub trait Event: Sized {
    /// Get the event's unique natural number identifier.
    fn id(&self) -> Result<u64, Error>;

    /// Send the event to Arvo and collect the result.
    fn play(self) -> Result<(), Error>;
}

/*=================================================================================================
 * Structs
 */

/// The event data structure.
pub struct UrEvent {
    id: u64,
}

/*=================================================================================================
 * Implementations
 */

impl Event for UrEvent {
    fn id(&self) -> Result<u64, Error> {
        Ok(self.id)
    }

    fn play(self) -> Result<(), Error> {
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
        assert!(true)
    }
}
