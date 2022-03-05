use crate::Noun;
use std::rc::Rc;

/// Pair of nouns.
#[derive(Debug)]
pub struct Cell {
    head: Rc<Noun>,
    tail: Rc<Noun>,
}

impl Cell {
    /// Create a new cell.
    #[allow(dead_code)]
    fn new(head: &Rc<Noun>, tail: &Rc<Noun>) -> Self {
        Self {
            head: Rc::clone(head),
            tail: Rc::clone(tail),
        }
    }

    /// Get the head of a cell.
    #[allow(dead_code)]
    pub fn h(&self) -> Rc<Noun> {
        Rc::clone(&self.head)
    }

    /// Get the tail of a cell.
    #[allow(dead_code)]
    pub fn t(&self) -> Rc<Noun> {
        Rc::clone(&self.tail)
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

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        if let (Noun::Atom(lh), Noun::Atom(rh)) = (&*self.head, &*other.head) {
            lh == rh && *self.tail == *other.tail
        } else if let (Noun::Cell(lh), Noun::Cell(rh)) = (&*self.head, &*other.head) {
            Self::eq(lh, rh) && *self.tail == *other.tail
        } else {
            false
        }
    }
}

/// Create a cell from a pair of &Rc<Noun>.
#[macro_export]
macro_rules! c {
    ($h:expr, $t:expr) => {
        Cell::new($h, $t)
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn clone() {
        // Clone [8 808].
        {}
    }

    #[test]
    fn partialeq() {
        // [71 109] == [71 109]
        {}

        // [71 109] != [109 71]
        {}

        // [11 [12 13]] == [11 [12 13]]
        {}

        // [11 [12 13]] != [11 [13 12]]
        {}
    }
}
