use crate::{
    hash::Mug,
    serdes::{Cue, Jam},
    Noun,
};
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
    pub fn new(head: &Rc<Noun>, tail: &Rc<Noun>) -> Self {
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

impl Mug for Cell {
    fn mug(&self) -> u32 {
        unimplemented!()
    }
}

impl Jam for Cell {
    fn jam(self) -> Vec<u8> {
        unimplemented!()
    }
}

impl Cue for Cell {
    fn cue(_bytes: Vec<u8>) -> Self {
        unimplemented!()
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

/// Shorthand for Cell::new(...).
#[macro_export]
macro_rules! c {
    ($head:expr, $tail:expr) => {
        Cell::new($head, $tail)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{atom::Atom, c, na, nc};

    #[test]
    fn clone() {
        // Clone [8 808].
        {
            let c = c!(&na![8], &na![808]);
            assert_eq!(c, c.clone());
        }
    }

    #[test]
    fn partialeq() {
        // [71 109] == [71 109]
        {
            let lh = c!(&na![71], &na![109]);
            let rh = c!(&na![71], &na![109]);
            assert_eq!(lh, rh);
        }

        // [71 109] != [109 71]
        {
            let lh = c!(&na![71], &na![109]);
            let rh = c!(&na![109], &na![71]);
            assert_ne!(lh, rh);
        }

        // [11 [12 13]] == [11 [12 13]]
        {
            let lh = c!(&na![11], &nc!(&na![12], &na![13]));
            let rh = c!(&na![11], &nc!(&na![12], &na![13]));
            assert_eq!(lh, rh);
        }

        // [11 [12 13]] != [11 [13 12]]
        {
            let lh = c!(&nc!(&na![11], &na![12]), &na![13]);
            let rh = c!(&na![11], &nc!(&na![12], &na![13]));
            assert_ne!(lh, rh);
        }
    }
}
