/// A Nock-specific boolean where 0 is yes/true and 1 is no/false.
#[derive(Debug, PartialEq)]
pub enum Loobean {
    Yes,
    No,
}

impl Loobean {
    /// Convert a boolean into a Loobean.
    #[allow(dead_code)]
    fn into_boolean(self) -> bool {
        Loobean::Yes == self
    }

    /// Convert a Loobean into a boolean.
    pub fn from_boolean(b: bool) -> Self {
        if b {
            Loobean::Yes
        } else {
            Loobean::No
        }
    }
}
