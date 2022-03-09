use crate::{
    atom::Atom,
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

/// Cell from (u64, u64).
impl From<(u64, u64)> for Cell {
    fn from((head, tail): (u64, u64)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (u64, non-empty Vec<u64>).
impl TryFrom<(u64, Vec<u64>)> for Cell {
    type Error = ();

    fn try_from((head, tail): (u64, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::try_from(tail)?),
        })
    }
}

/// Cell from (u64, Atom).
impl From<(u64, Atom)> for Cell {
    fn from((head, tail): (u64, Atom)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (u64, Cell).
impl From<(u64, Cell)> for Cell {
    fn from((head, tail): (u64, Cell)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (u64, Noun).
impl From<(u64, Noun)> for Cell {
    fn from((head, tail): (u64, Noun)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(tail),
        }
    }
}

/// Cell from (u64, Rc<Noun>).
impl From<(u64, Rc<Noun>)> for Cell {
    fn from((head, tail): (u64, Rc<Noun>)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::clone(&tail),
        }
    }
}

/// Cell from (non-empty Vec<u64>, u64).
impl TryFrom<(Vec<u64>, u64)> for Cell {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, u64)) -> Result<Self, Self::Error> {
        Ok(Self {
            head: Rc::new(Noun::try_from(head)?),
            tail: Rc::new(Noun::from(tail)),
        })
    }
}

/// Cell from (non-empty Vec<u64>, non-empty Vec<u64>).
impl TryFrom<(Vec<u64>, Vec<u64>)> for Cell {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self {
            head: Rc::new(Noun::try_from(head)?),
            tail: Rc::new(Noun::try_from(tail)?),
        })
    }
}

/// Cell from (non-empty Vec<u64>, Atom).
impl TryFrom<(Vec<u64>, Atom)> for Cell {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, Atom)) -> Result<Self, Self::Error> {
        Ok(Self {
            head: Rc::new(Noun::try_from(head)?),
            tail: Rc::new(Noun::from(tail)),
        })
    }
}

/// Cell from (non-empty Vec<u64>, Cell).
impl TryFrom<(Vec<u64>, Cell)> for Cell {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, Cell)) -> Result<Self, Self::Error> {
        Ok(Self {
            head: Rc::new(Noun::try_from(head)?),
            tail: Rc::new(Noun::from(tail)),
        })
    }
}

/// Cell from (non-empty Vec<u64>, Noun).
impl TryFrom<(Vec<u64>, Noun)> for Cell {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, Noun)) -> Result<Self, Self::Error> {
        Ok(Self {
            head: Rc::new(Noun::try_from(head)?),
            tail: Rc::new(tail),
        })
    }
}

/// Cell from (non-empty Vec<u64>, Rc<Noun>).
impl TryFrom<(Vec<u64>, Rc<Noun>)> for Cell {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, Rc<Noun>)) -> Result<Self, Self::Error> {
        Ok(Self {
            head: Rc::new(Noun::try_from(head)?),
            tail: Rc::clone(&tail),
        })
    }
}

/// Cell from (Atom, u64).
impl From<(Atom, u64)> for Cell {
    fn from((head, tail): (Atom, u64)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Atom, non-empty Vec<u64>).
impl TryFrom<(Atom, Vec<u64>)> for Cell {
    type Error = ();

    fn try_from((head, tail): (Atom, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::try_from(tail)?),
        })
    }
}

/// Cell from (Atom, Atom).
impl From<(Atom, Atom)> for Cell {
    fn from((head, tail): (Atom, Atom)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Atom, Cell).
impl From<(Atom, Cell)> for Cell {
    fn from((head, tail): (Atom, Cell)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Atom, Noun).
impl From<(Atom, Noun)> for Cell {
    fn from((head, tail): (Atom, Noun)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(tail),
        }
    }
}

/// Cell from (Atom, Rc<Noun>).
impl From<(Atom, Rc<Noun>)> for Cell {
    fn from((head, tail): (Atom, Rc<Noun>)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::clone(&tail),
        }
    }
}

/// Cell from (Cell, u64).
impl From<(Cell, u64)> for Cell {
    fn from((head, tail): (Cell, u64)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Cell, non-empty Vec<u64>).
impl TryFrom<(Cell, Vec<u64>)> for Cell {
    type Error = ();

    fn try_from((head, tail): (Cell, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::try_from(tail)?),
        })
    }
}

/// Cell from (Cell, Atom).
impl From<(Cell, Atom)> for Cell {
    fn from((head, tail): (Cell, Atom)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Cell, Cell).
impl From<(Cell, Cell)> for Cell {
    fn from((head, tail): (Cell, Cell)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Cell, Noun).
impl From<(Cell, Noun)> for Cell {
    fn from((head, tail): (Cell, Noun)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::new(tail),
        }
    }
}

/// Cell from (Cell, Rc<Noun>).
impl From<(Cell, Rc<Noun>)> for Cell {
    fn from((head, tail): (Cell, Rc<Noun>)) -> Self {
        Self {
            head: Rc::new(Noun::from(head)),
            tail: Rc::clone(&tail),
        }
    }
}

/// Cell from (Noun, u64).
impl From<(Noun, u64)> for Cell {
    fn from((head, tail): (Noun, u64)) -> Self {
        Self {
            head: Rc::new(head),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Noun, non-empty Vec<u64>).
impl TryFrom<(Noun, Vec<u64>)> for Cell {
    type Error = ();

    fn try_from((head, tail): (Noun, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self {
            head: Rc::new(head),
            tail: Rc::new(Noun::try_from(tail)?),
        })
    }
}

/// Cell from (Noun, Atom).
impl From<(Noun, Atom)> for Cell {
    fn from((head, tail): (Noun, Atom)) -> Self {
        Self {
            head: Rc::new(head),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Noun, Cell).
impl From<(Noun, Cell)> for Cell {
    fn from((head, tail): (Noun, Cell)) -> Self {
        Self {
            head: Rc::new(head),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Noun, Noun).
impl From<(Noun, Noun)> for Cell {
    fn from((head, tail): (Noun, Noun)) -> Self {
        Self {
            head: Rc::new(head),
            tail: Rc::new(tail),
        }
    }
}

/// Cell from (Noun, Rc<Noun>).
impl From<(Noun, Rc<Noun>)> for Cell {
    fn from((head, tail): (Noun, Rc<Noun>)) -> Self {
        Self {
            head: Rc::new(head),
            tail: Rc::clone(&tail),
        }
    }
}

/// Cell from (Rc<Noun>, u64).
impl From<(Rc<Noun>, u64)> for Cell {
    fn from((head, tail): (Rc<Noun>, u64)) -> Self {
        Self {
            head: Rc::clone(&head),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Rc<Noun>, non-empty Vec<u64>).
impl TryFrom<(Rc<Noun>, Vec<u64>)> for Cell {
    type Error = ();

    fn try_from((head, tail): (Rc<Noun>, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self {
            head: Rc::clone(&head),
            tail: Rc::new(Noun::try_from(tail)?),
        })
    }
}

/// Cell from (Rc<Noun>, Atom).
impl From<(Rc<Noun>, Atom)> for Cell {
    fn from((head, tail): (Rc<Noun>, Atom)) -> Self {
        Self {
            head: Rc::clone(&head),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Rc<Noun>, Cell).
impl From<(Rc<Noun>, Cell)> for Cell {
    fn from((head, tail): (Rc<Noun>, Cell)) -> Self {
        Self {
            head: Rc::clone(&head),
            tail: Rc::new(Noun::from(tail)),
        }
    }
}

/// Cell from (Rc<Noun>, Noun).
impl From<(Rc<Noun>, Noun)> for Cell {
    fn from((head, tail): (Rc<Noun>, Noun)) -> Self {
        Self {
            head: Rc::clone(&head),
            tail: Rc::new(tail),
        }
    }
}

/// Cell from (Rc<Noun>, Rc<Noun>).
impl From<(Rc<Noun>, Rc<Noun>)> for Cell {
    fn from((head, tail): (Rc<Noun>, Rc<Noun>)) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone() -> Result<(), ()> {
        // Clone [8 808].
        {
            let cell = Cell::from((8, 808));
            assert_eq!(cell.clone(), cell);
        }

        Ok(())
    }

    #[test]
    fn partialeq() -> Result<(), ()> {
        // [71 109] == [71 109]
        {
            let lh = Cell::from((71, 109));
            let rh = Cell::from((71, 109));
            assert_eq!(lh, rh);
        }

        // [71 109] != [109 71]
        {
            let lh = Cell::from((71, 109));
            let rh = Cell::from((109, 71));
            assert_ne!(lh, rh);
        }

        // [11 [12 13]] == [11 [12 13]]
        {
            let lh = Cell::from((11, Cell::from((12, 13))));
            let rh = Cell::from((11, Cell::from((12, 13))));
            assert_eq!(lh, rh);
        }

        // [11 [12 13]] != [11 [13 12]]
        {
            let lh = Cell::from((11, Cell::from((12, 13))));
            let rh = Cell::from((11, Cell::from((13, 12))));
            assert_ne!(lh, rh);
        }

        Ok(())
    }
}
