use crate::{atom::Atom, cell::Cell, hash::Mug};
use std::{fmt, rc::Rc};

pub mod atom;
pub mod cell;
pub mod hash;
pub mod serdes;

/// Atom or a cell.
#[derive(Debug)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

impl Noun {
    /// Create a new reference-counted atom.
    #[allow(dead_code)]
    pub fn new_atom(atom: Atom) -> Rc<Noun> {
        Rc::new(Noun::Atom(atom))
    }

    /// Create a new reference-counted cell.
    #[allow(dead_code)]
    pub fn new_cell(cell: Cell) -> Rc<Noun> {
        Rc::new(Noun::Cell(cell))
    }
}

impl Mug for Noun {
    fn mug(&self) -> u32 {
        match self {
            Self::Atom(a) => a.mug(),
            Self::Cell(c) => c.mug(),
        }
    }
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

/// Shorthand for Noun::new_atom(Atom::new(vec![...])).
#[macro_export]
macro_rules! na {
    ( $( $elem:expr ),+ ) => {
        {
        let mut temp_vec: Vec<u64> = Vec::new();
        $(
            temp_vec.push($elem);
        )*
        Noun::new_atom(Atom::new(temp_vec))
        }
    };
}

/// Shorthand for Noun::new_cell(Cell::new(...)).
#[macro_export]
macro_rules! nc {
    ($head:expr, $tail:expr) => {
        Noun::new_cell(Cell::new($head, $tail))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone() {
        // Clone 101010.
        {
            let a = na![10101];
            assert_eq!(a, a.clone());
        }

        // Clone [300 [400 500]].
        {
            let c = nc!(&na![300], &nc!(&na![400], &na![500]));
            assert_eq!(c, c.clone());
        }
    }

    #[test]
    fn partialeq() {
        // [0 5] == [0 5]
        {
            let lh = nc!(&na![0], &na![5]);
            let rh = nc!(&na![0], &na![5]);
            assert_eq!(lh, rh);
        }

        // [0 0] == [0 5]
        {
            let lh = nc!(&na![0], &na![5]);
            let rh = nc!(&na![5], &na![0]);
            assert_ne!(lh, rh);
        }

        // [[44 22] 88] == [[44 22] 88]
        {
            let lh = nc!(&nc!(&na![44], &na![22]), &na![88]);
            let rh = nc!(&nc!(&na![44], &na![22]), &na![88]);
            assert_eq!(lh, rh);
        }

        // [[44 22] 88] != [44 [22 88]]
        {
            let lh = nc!(&nc!(&na![44], &na![22]), &na![88]);
            let rh = nc!(&na![44], &nc!(&na![22], &na![88]));
            assert_ne!(lh, rh);
        }
    }
}
