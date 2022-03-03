use std::rc::Rc;

/// Arbitrarily large unsigned integer.
#[derive(Clone, Debug, PartialEq)]
pub struct Atom {
    val: Vec<u64>,
}

impl Atom {
    /// Create a new reference-counted atom.
    #[allow(dead_code)]
    fn new(val: Vec<u64>) -> Rc<Atom> {
        Rc::new(Atom { val })
    }

    /// Get the value of an atom.
    #[allow(dead_code)]
    fn v(&self) -> &Vec<u64> {
        &self.val
    }
}

/*
/// Get an atom's value.
macro_rules! av {
    ($a:expr) => {
        match $a {
            crate::atom::Atom::Direct(v) => v,
            crate::atom::Atom::Indirect(_) => unimplemented!(),
        }
    };
}
*/

/// Create a new reference-counted atom from a variadic list of u64.
#[macro_export]
macro_rules! a {
    ( $( $elem:expr ),+ ) => {
        {
            let mut temp_vec: Vec<u64> = Vec::new();
            $(
                temp_vec.push($elem);
             )*
            crate::atom::Atom::new(temp_vec)
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

        // Clone a very large number.
        {
            let a = a![u64::MAX, u64::MAX, u64::MAX, u64::MAX];
            assert_eq!(a, a.clone());
        }
    }
}
