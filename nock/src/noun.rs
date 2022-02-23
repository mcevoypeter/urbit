use crate::{a, atom::Atom, c, cell::Cell, loobean::Loobean, na, nc};
use std::fmt;

/// An atom or a cell.
///
/// This wraps the Atom and Cell structs to support functions that need to return a noun but don't
/// know whether that noun is an atom or a cell.
#[derive(Debug)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

impl Clone for Noun {
    fn clone(&self) -> Self {
        match self {
            Noun::Atom(a) => na!(a.0),
            Noun::Cell(c) => nc!(c.h.clone(), c.t.clone()),
        }
    }
}

impl PartialEq for Noun {
    fn eq(&self, other: &Self) -> bool {
        if let (Noun::Atom(lh), Noun::Atom(rh)) = (&*self, &*other) {
            lh == rh
        } else if let (Noun::Cell(lh), Noun::Cell(rh)) = (&*self, &*other) {
            lh == rh
        } else {
            false
        }
    }
}

impl fmt::Display for Noun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Noun::Atom(a) => {
                write!(f, "{}", a.0)
            }
            Noun::Cell(ref c) => {
                write!(f, "[{} {}]", c.h, c.t)
            }
        }
    }
}

impl Noun {
    pub fn new_atom(v: u64) -> Self {
        Noun::Atom(a!(v))
    }

    pub fn new_cell(h: Box<Self>, t: Box<Self>) -> Self {
        Noun::Cell(c!(h, t))
    }

    pub fn from_loobean(l: Loobean) -> Self {
        match l {
            Loobean::Yes => Noun::new_atom(0),
            Loobean::No => Noun::new_atom(1),
        }
    }

    pub fn into_box(self) -> Box<Self> {
        Box::new(self)
    }
}

/// Noun::new_atom($v)
#[macro_export]
macro_rules! na {
    ($v:expr) => {
        Noun::new_atom($v)
    };
}

/// Noun::new_cell($h, $t)
#[macro_export]
macro_rules! nc {
    ($h:expr, $t:expr) => {
        Noun::new_cell($h, $t)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b;

    #[test]
    fn clone() {
        // Clone 101010.
        {
            let noun = na!(101010);
            assert_eq!(noun, noun.clone());
        }

        // Clone [300 [400 500]].
        {
            let noun = nc!(b!(na!(300)), b!(nc!(b!(na!(400)), b!(na!(500)))));
            assert_eq!(noun, noun.clone());
        }
    }

    #[test]
    fn partialeq() {
        // 500 == 500
        {
            assert_eq!(a!(500), a!(500));
        }

        // 499 != 501
        {
            assert_ne!(a!(499), a!(501));
        }

        // [0 5] == [0 5]
        {
            assert_eq!(nc!(b!(na!(0)), b!(na!(5))), nc!(b!(na!(0)), b!(na!(5))));
        }

        // [0 0] == [0 5]
        {
            assert_ne!(nc!(b!(na!(0)), b!(na!(0))), nc!(b!(na!(0)), b!(na!(5))));
        }

        // [[44 22] 88] == [[44 22] 88]
        {
            assert_eq!(
                nc!(b!(nc!(b!(na!(44)), b!(na!(22)))), b!(na!(88))),
                nc!(b!(nc!(b!(na!(44)), b!(na!(22)))), b!(na!(88)))
            );
        }

        // [[44 22] 88] != [44 [22 88]]
        {
            assert_ne!(
                nc!(b!(nc!(b!(na!(44)), b!(na!(22)))), b!(na!(88))),
                nc!(b!(na!(44)), b!(nc!(b!(na!(22)), b!(na!(88)))))
            );
        }
    }
}