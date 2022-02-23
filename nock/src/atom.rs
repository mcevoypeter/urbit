/// An unsigned integer.
#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Direct(u64),
    Indirect(Vec<u64>),
}

/// Create an atom.
#[macro_export]
macro_rules! a {
    ($v:expr) => {
        crate::atom::Atom::Direct($v)
    };
}

/// Get an atom's value.
macro_rules! av {
    ($a:expr) => {
        match $a {
            crate::atom::Atom::Direct(v) => v,
            crate::atom::Atom::Indirect(_) => unimplemented!(),
        }
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
