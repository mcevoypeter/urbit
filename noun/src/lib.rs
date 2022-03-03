use crate::{atom::Atom, cell::Cell};
use std::rc::Rc;

pub mod atom;
pub mod cell;

/// Atom or a cell.
#[derive(Debug)]
pub enum Noun {
    Atom(Rc<Atom>),
    Cell(Rc<Cell>),
}

/*
impl Clone for Noun {
    fn clone(&self) -> Self {
        match self {
            Noun::Atom(a) => Noun::Atom(a!(av!(*a))),
            Noun::Cell(c) => Noun::Cell(c!(ch!(c).clone(), ct!(c).clone())),
        }
    }
}
*/

/*
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
*/

/*
impl fmt::Display for Noun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Noun::Atom(a) => {
                write!(f, "{}", av!(a))
            }
            Noun::Cell(ref c) => {
                write!(f, "[{} {}]", ch!(c), ct!(c))
            }
        }
    }
}
*/

/*
#[macro_export]
macro_rules! b {
    ($e:expr) => {
        std::rc::Rc::clone($e)
    }
}
*/

/*
/// Create a noun-wrapped atom.
#[macro_export]
macro_rules! na {
    ($v:expr) => {
        crate::Noun::Atom(a!($v))
    };
}
*/

/*
/// Create a noun-wrapped cell.
#[macro_export]
macro_rules! nc {
    ($h:expr, $t:expr) => {
        crate::Noun::Cell(c!($h, $t))
    };
}
*/

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
*/
