use crate::{
    hash::Mug,
    serdes::{Cue, Jam},
    Noun,
};
use std::fmt;

/// Arbitrarily large unsigned integer.
#[derive(Clone, Debug, PartialEq)]
pub struct Atom {
    val: Vec<u64>,
}

impl Atom {
    /// Get the value of an atom.
    #[allow(dead_code)]
    pub fn v(&self) -> &Vec<u64> {
        &self.val
    }
}

impl Mug for Atom {
    fn mug(&self) -> u32 {
        unimplemented!()
    }
}

impl Jam for Atom {
    fn jam(self) -> Vec<u8> {
        unimplemented!()
    }
}

impl Cue for Atom {
    fn cue(_bytes: Vec<u8>) -> Self {
        unimplemented!()
    }
}

/// Atom from u64.
impl From<u64> for Atom {
    fn from(val: u64) -> Self {
        Self { val: vec![val] }
    }
}

/// Atom from non-empty Vec<u64>.
impl TryFrom<Vec<u64>> for Atom {
    type Error = ();

    fn try_from(val: Vec<u64>) -> Result<Self, Self::Error> {
        if !val.is_empty() {
            Ok(Self { val })
        } else {
            Err(())
        }
    }
}

/// Atom from Noun.
impl TryFrom<Noun> for Atom {
    type Error = ();

    fn try_from(noun: Noun) -> Result<Self, Self::Error> {
        if let Noun::Atom(atom) = noun {
            Ok(atom)
        } else {
            Err(())
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone() -> Result<(), ()> {
        // Clone 777.
        {
            let a = Atom::from(777);
            assert_eq!(a.clone(), a);
        }

        // Clone 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff.
        {
            let a = Atom::from(u64::MAX);
            assert_eq!(a.clone(), a);
        }

        // Clone 2^64.
        {
            let a = Atom::try_from(vec![0, 1])?;
            assert_eq!(a.clone(), a);
        }

        // Clone a really big number.
        {
            let a = Atom::try_from(vec![0, 1, 2, 3, 4, 5, 6, 7]);
            assert_eq!(a.clone(), a);
        }

        Ok(())
    }

    #[test]
    fn partialeq() -> Result<(), ()> {
        // 500 == 500
        {
            let lh = Atom::from(500);
            let rh = Atom::from(500);
            assert_eq!(lh, rh)
        }

        // 499 != 501
        {
            let lh = Atom::from(499);
            let rh = Atom::from(501);
            assert_ne!(lh, rh)
        }

        // 2^64 == 2^64.
        {
            let lh = Atom::try_from(vec![0, 1])?;
            let rh = Atom::try_from(vec![0, 1])?;
            assert_eq!(lh, rh);
        }

        // 2^64 != 2^65
        {
            let lh = Atom::try_from(vec![0, 1])?;
            let rh = Atom::try_from(vec![0, 2])?;
            assert_ne!(lh, rh);
        }

        Ok(())
    }
}
