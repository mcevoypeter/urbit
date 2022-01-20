mod event;
mod event_log;
mod snapshot;

/*=================================================================================================
 * Structs
 */

/// A Vere-specific error encapsulating an informative error message.
#[derive(Debug)]
pub struct Error {
    msg: String,
}
