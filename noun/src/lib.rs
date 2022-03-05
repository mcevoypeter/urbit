use crate::{atom::Atom, cell::Cell};
use std::{fmt, rc::Rc};

#[macro_use]
pub mod atom;
pub mod cell;

/// Atom or a cell.
#[derive(Debug)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
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

impl Noun {
    /// Create a new reference-counted atom.
    #[allow(dead_code)]
    fn new_atom(atom: Atom) -> Rc<Noun> {
        Rc::new(Noun::Atom(atom))
    }

    /// Create a new reference-counted cell.
    #[allow(dead_code)]
    fn new_cell(cell: Cell) -> Rc<Noun> {
        Rc::new(Noun::Cell(cell))
    }
}

/// Shorthand for Noun::new_atom().
#[macro_export]
macro_rules! na {
    ($atom:expr) => {
        Noun::new_atom($atom)
    };
}

/// Shorthand for Noun::new_cell().
#[macro_export]
macro_rules! nc {
    ($cell:expr) => {
        Noun::new_cell($cell)
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn clone() {
        // Clone 101010.
        {}

        // Clone [300 [400 500]].
        {}
    }

    #[test]
    fn partialeq() {
        // [0 5] == [0 5]
        {}

        // [0 0] == [0 5]
        {}

        // [[44 22] 88] == [[44 22] 88]
        {}

        // [[44 22] 88] != [44 [22 88]]
        {}
    }
}
