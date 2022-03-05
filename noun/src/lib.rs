use crate::{atom::Atom, cell::Cell};
use std::{fmt, rc::Rc};

#[macro_use]
pub mod atom;
pub mod cell;

/// Atom or a cell.
#[derive(Debug)]
pub enum Noun {
    Atom(Rc<Atom>),
    Cell(Rc<Cell>),
}

impl Clone for Noun {
    fn clone(&self) -> Self {
        match self {
            Self::Atom(a) => Self::Atom(a.clone()),
            Self::Cell(c) => Self::Cell(c.clone()),
        }
    }
}

impl PartialEq for Noun {
    fn eq(&self, other: &Self) -> bool {
        if let (Self::Atom(lh), Self::Atom(rh)) = (&*self, &*other) {
            lh == rh
        } else if let (Self::Cell(lh), Self::Cell(rh)) = (&*self, &*other) {
            *lh == *rh
        } else {
            false
        }
    }
}

impl fmt::Display for Noun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(a) => {
                write!(f, "{}", a)
            }
            Self::Cell(c) => {
                write!(f, "[{} {}]", c.h(), c.t())
            }
        }
    }
}

/// Wrap an atom in a noun.
#[macro_export]
macro_rules! na {
    ($atom:expr) => {
        Noun::Atom($atom)
    };
}

/// Wrap a cell in a noun.
#[macro_export]
macro_rules! nc {
    ($cell:expr) => {
        Noun::Cell($cell)
    };
}

/*
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
*/
