use crate::{atom::Atom, Error, Noun};
use std::{fmt, rc::Rc};

/// Pair of nouns.
pub struct Cell {
    head: Rc<dyn Noun>,
    tail: Rc<dyn Noun>,
}

impl Cell {
    /// Get the head of a cell.
    #[allow(dead_code)]
    pub fn head(&self) -> Rc<dyn Noun> {
        Rc::clone(&self.head)
    }

    /// Get the tail of a cell.
    #[allow(dead_code)]
    pub fn tail(&self) -> Rc<dyn Noun> {
        Rc::clone(&self.tail)
    }
}

impl Noun for Cell {
    fn is_atom(&self) -> bool {
        false
    }

    fn is_cell(&self) -> bool {
        true
    }

    fn as_atom(&self) -> Result<&Atom, Error> {
        Err(Error::UnexpectedCell)
    }

    fn as_cell(&self) -> Result<&Cell, Error> {
        Ok(self)
    }

    fn into_atom(self) -> Result<Atom, Error> {
        Err(Error::UnexpectedCell)
    }

    fn into_cell(self) -> Result<Cell, Error> {
        Ok(self)
    }
}

impl From<(Atom, Atom)> for Cell {
    fn from((head, tail): (Atom, Atom)) -> Self {
        Self {
            head: Rc::new(head),
            tail: Rc::new(tail),
        }
    }
}

impl From<(Atom, Cell)> for Cell {
    fn from((head, tail): (Atom, Self)) -> Self {
        Self {
            head: Rc::new(head),
            tail: Rc::new(tail),
        }
    }
}

impl From<(Cell, Atom)> for Cell {
    fn from((head, tail): (Self, Atom)) -> Self {
        Self {
            head: Rc::new(head),
            tail: Rc::new(tail),
        }
    }
}

impl From<(Cell, Cell)> for Cell {
    fn from((head, tail): (Self, Self)) -> Self {
        Self {
            head: Rc::new(head),
            tail: Rc::new(tail),
        }
    }
}

impl From<(Rc<dyn Noun>, Rc<dyn Noun>)> for Cell {
    fn from((head, tail): (Rc<dyn Noun>, Rc<dyn Noun>)) -> Self {
        Self {
            head: Rc::clone(&head),
            tail: Rc::clone(&tail),
        }
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Self {
            head: self.head.clone(),
            tail: self.tail.clone(),
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Ok(())
    }
}

/// Determine if two Noun objects match.
fn fields_match(left: &Rc<dyn Noun>, right: &Rc<dyn Noun>) -> bool {
    if let (Ok(l), Ok(r)) = (left.as_atom(), right.as_atom()) {
        l == r
    } else if let (Ok(l), Ok(r)) = (left.as_cell(), right.as_cell()) {
        Cell::eq(l, r)
    } else {
        false
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        fields_match(&self.head, &other.head) && fields_match(&self.tail, &other.tail)
    }
}

#[macro_export]
macro_rules! c {
    ( $head:expr, $tail:expr ) => {
        Cell::from(($head, $tail))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone() {
        // Clone [8 808].
        {
            let c = c!(a![8], a![808]);
            assert_eq!(c.clone(), c);
        }
    }

    #[test]
    fn partialeq() {
        // [71 109] == [71 109]
        {
            let lh = c!(a![71], a![109]);
            let rh = c!(a![71], a![109]);
            assert_eq!(lh, rh);
        }

        // [71 109] != [109 71]
        {
            let lh = c!(a![71], a![109]);
            let rh = c!(a![109], a![71]);
            assert_ne!(lh, rh);
        }

        // [11 [12 13]] == [11 [12 13]]
        {
            let lh = c!(a![11], c!(a![12], a![13]));
            let rh = c!(a![11], c!(a![12], a![13]));
            assert_eq!(lh, rh);
        }

        // [11 [12 13]] != [11 [13 12]]
        {
            let lh = c!(a![11], c!(a![12], a![13]));
            let rh = c!(a![11], c!(a![13], a![12]));
            assert_ne!(lh, rh);
        }
    }
}
