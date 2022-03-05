use crate::{atom::Atom, Noun};
use std::{fmt, rc::Rc};

/// Pair of nouns.
pub struct Cell {
    head: Rc<dyn Noun>,
    tail: Rc<dyn Noun>,
}

impl Cell {
    /// Create a new reference-counted cell.
    #[allow(dead_code)]
    fn new(head: &Rc<dyn Noun>, tail: &Rc<dyn Noun>) -> Rc<Self> {
        Rc::new(Self {
            head: Rc::clone(head),
            tail: Rc::clone(tail),
        })
    }

    /// Get the head of a cell.
    #[allow(dead_code)]
    pub fn h(&self) -> Rc<dyn Noun> {
        Rc::clone(&self.head)
    }

    /// Get the tail of a cell.
    #[allow(dead_code)]
    pub fn t(&self) -> Rc<dyn Noun> {
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

    fn as_atom(&self) -> Result<&Atom, ()> {
        Err(())
    }

    fn as_cell(&self) -> Result<&Cell, ()> {
        Ok(self)
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

/// Determine if the tails of two cells match.
macro_rules! tails_match {
    ( $left:expr, $right:expr ) => {
        if let (Ok(lt), Ok(rt)) = ($left.tail.as_atom(), $right.tail.as_atom()) {
            lt == rt
        } else if let (Ok(lt), Ok(rt)) = ($left.tail.as_cell(), $right.tail.as_cell()) {
            Cell::eq(lt, rt)
        } else {
            false
        }
    };
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        if let (Ok(sh), Ok(oh)) = (self.head.as_atom(), other.head.as_atom()) {
            sh == oh && tails_match!(self, other)
        } else if let (Ok(sh), Ok(oh)) = (self.head.as_cell(), other.head.as_cell()) {
            Self::eq(sh, oh) && tails_match!(self, other)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone() {
        // Clone [8 808].
    }

    #[test]
    fn partialeq() {
        // [71 109] == [71 109]

        // [71 109] != [109 71]

        // [11 [12 13]] == [11 [12 13]]

        // [11 [12 13]] != [11 [13 12]]
    }
}
