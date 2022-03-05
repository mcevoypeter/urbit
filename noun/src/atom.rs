use std::{fmt, rc::Rc};

/// Arbitrarily large unsigned integer.
#[derive(Clone, Debug, PartialEq)]
pub struct Atom {
    val: Vec<u64>,
}

impl Atom {
    /// Create a new reference-counted atom.
    #[allow(dead_code)]
    pub fn new(val: Vec<u64>) -> Rc<Self> {
        Rc::new(Self { val })
    }

    /// Get the value of an atom.
    #[allow(dead_code)]
    pub fn v(&self) -> &Vec<u64> {
        &self.val
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = "0x";
        let mut string = String::from(prefix);
        for num in self.v() {
            let num_string = &format!("{:#016x}_", num)[prefix.len()..];
            string += num_string;
        }
        string.pop(); // remove trailing _
        write!(f, "{}", string)
    }
}

/// Create a new reference-counted atom from a variadic list of u64.
#[macro_export]
macro_rules! a {
    ( $( $elem:expr ),+ ) => {
        {
            let mut temp_vec: Vec<u64> = Vec::new();
            $(
                temp_vec.push($elem);
             )*
            noun::atom::Atom::new(temp_vec)
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn clone() {
        // Clone 777.
        {
            let a = a![777];
            assert_eq!(a, a.clone());
        }

        // Clone 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff.
        {
            let a = a![u64::MAX, u64::MAX, u64::MAX, u64::MAX];
            assert_eq!(a, a.clone());
        }
    }

    #[test]
    fn partialeq() {
        // 500 == 500
        {
            assert_eq!(a![500], a![500]);
        }

        // 499 != 501
        {
            assert_ne!(a![499], a![501]);
        }

        // 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff == 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff
        {
            assert_eq!(a![u64::MAX, u64::MAX], a![u64::MAX, u64::MAX]);
        }

        // 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff != 0xffff_ffff_ffff_ffff_0000_0000_0000_0000
        {
            assert_ne!(a![u64::MAX, u64::MAX], a![u64::MAX, 0]);
        }
    }
}
