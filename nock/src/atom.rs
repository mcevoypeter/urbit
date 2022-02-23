/// An unsigned integer.
#[derive(Clone, Debug, PartialEq)]
pub struct Atom(pub u64);

/// Create an atom.
#[macro_export]
macro_rules! a {
    ($v:expr) => {
        crate::atom::Atom($v)
    };
}

/// Get an atom's value.
macro_rules! av {
    ($a:expr) => {
        $a.0
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
