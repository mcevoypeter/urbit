use crate::{cell::Cell, Noun};
use std::fmt;

/// Arbitrarily large unsigned integer.
#[derive(Clone, Debug, PartialEq)]
pub struct Atom {
    val: Vec<u64>,
}

impl Atom {
    #[allow(dead_code)]
    pub fn v(&self) -> &Vec<u64> {
        &self.val
    }
}

impl Noun for Atom {
    fn is_atom(&self) -> bool {
        true
    }

    fn is_cell(&self) -> bool {
        false
    }

    fn as_atom(&self) -> Result<&Atom, ()> {
        Ok(self)
    }

    fn as_cell(&self) -> Result<&Cell, ()> {
        Err(())
    }

    fn into_atom(self) -> Result<Atom, ()> {
        Ok(self)
    }

    fn into_cell(self) -> Result<Cell, ()> {
        Err(())
    }
}

impl From<u64> for Atom {
    fn from(val: u64) -> Self {
        Atom { val: vec![val] }
    }
}

impl From<Vec<u64>> for Atom {
    fn from(mut val: Vec<u64>) -> Self {
        if 0 == val.len() {
            val.push(0)
        }
        Self { val }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = "0x";
        let mut string = String::from(prefix);
        for num in &self.val {
            let num_string = &format!("{:#016x}_", num)[prefix.len()..];
            string += num_string;
        }
        string.pop(); // remove trailing _
        write!(f, "{}", string)
    }
}

#[macro_export]
macro_rules! a {
    ( $elem:expr ) => {
        Atom::from($elem)
    };
    ( $( $elem:expr ),+ ) => {
        {
            let mut temp_vec: Vec<u64> = Vec::new();
            $(
                temp_vec.push($elem);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone() {
        // Clone 777.
        {
            let a = a![777];
            assert_eq!(a.clone(), a);
        }

        // Clone 2^64.
        {
            let a = a![0, 1];
            assert_eq!(a.clone(), a);
        }
    }

    #[test]
    fn partialeq() {
        // 500 == 500
        {
            let lh = a![500];
            let rh = a![500];
            assert_eq!(lh, rh);
        }

        // 499 != 501
        {
            let lh = a![499];
            let rh = a![501];
            assert_ne!(lh, rh);
        }

        // 2^64 == 2^64
        {
            let lh = a![0, 1];
            let rh = a![0, 1];
            assert_eq!(lh, rh);
        }

        // 2^64 != 2^65
        {
            let lh = a![0, 1];
            let rh = a![0, 2];
            assert_ne!(lh, rh);
        }
    }
}
