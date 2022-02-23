/// An unsigned integer.
#[derive(Clone, Debug, PartialEq)]
pub struct Atom(pub u64);

impl Atom {
    pub fn value(&self) -> u64 {
        self.0
    }
}

/// Create an atom.
#[macro_export]
macro_rules! a {
    ($v:expr) => {
        crate::atom::Atom($v)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn clone() {
        // Clone 777.
        {
            let a = a!(777);
            assert_eq!(a, a.clone());
        }
    }
}
