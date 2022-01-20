use crate::storage::Snapshot;

/*=================================================================================================
 * Structs
 */

/// The snapshot data structure.
struct UrSnapshot;

/*=================================================================================================
 * Implementations
 */

impl Snapshot for UrSnapshot {}


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
