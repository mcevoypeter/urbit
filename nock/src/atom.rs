/// An unsigned integer.
#[derive(Clone, Debug, PartialEq)]
pub struct Atom(pub u64);

impl Atom {
    pub fn new(v: u64) -> Self {
        Atom(v)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

/// Atom::new($v)
#[macro_export]
macro_rules! a {
    ($v:expr) => {
        Atom::new($v)
    };
}
