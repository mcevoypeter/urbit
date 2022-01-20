mod event;
mod storage;

/*=================================================================================================
 * Structs
 */

/// A Vere-specific error encapsulating an informative error message.
#[derive(Debug)]
pub struct Error {
    msg: String,
}
