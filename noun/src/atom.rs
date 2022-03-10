use crate::{cell::Cell, Noun};
use std::fmt;

/// Arbitrarily large unsigned integer.
#[derive(Clone, Debug, PartialEq)]
pub struct Atom {
    val: Vec<u64>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone() {
        // Clone 777.

        // Clone 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff.
    }

    #[test]
    fn partialeq() {
        // 500 == 500

        // 499 != 501

        // 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff == 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff

        // 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff != 0xffff_ffff_ffff_ffff_0000_0000_0000_0000
    }
}
