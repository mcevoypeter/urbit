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

impl Mug for Noun {
    fn mug(&self) -> u32 {
        match self {
            Self::Atom(a) => a.mug(),
            Self::Cell(c) => c.mug(),
        }
    }
}

/// Noun from u64.
impl From<u64> for Noun {
    fn from(val: u64) -> Self {
        Noun::Atom(Atom::from(val))
    }
}

/// Noun from non-empty Vec<u64>.
impl TryFrom<Vec<u64>> for Noun {
    type Error = ();

    fn try_from(val: Vec<u64>) -> Result<Self, Self::Error> {
        Ok(Noun::Atom(Atom::try_from(val)?))
    }
}

/// Noun from Atom.
impl From<Atom> for Noun {
    fn from(atom: Atom) -> Self {
        Noun::Atom(atom)
    }
}

/// Noun from Cell.
impl From<Cell> for Noun {
    fn from(cell: Cell) -> Self {
        Noun::Cell(cell)
    }
}

/// Noun from (u64, u64).
impl From<(u64, u64)> for Noun {
    fn from((head, tail): (u64, u64)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (u64, non-empty Vec<u64>).
impl TryFrom<(u64, Vec<u64>)> for Noun {
    type Error = ();

    fn try_from((head, tail): (u64, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self::from(Cell::try_from((head, tail))?))
    }
}

/// Noun from (u64, Atom).
impl From<(u64, Atom)> for Noun {
    fn from((head, tail): (u64, Atom)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (u64, Cell).
impl From<(u64, Cell)> for Noun {
    fn from((head, tail): (u64, Cell)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (u64, Noun).
impl From<(u64, Noun)> for Noun {
    fn from((head, tail): (u64, Noun)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (u64, Rc<Noun>).
impl From<(u64, Rc<Noun>)> for Noun {
    fn from((head, tail): (u64, Rc<Noun>)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (non-empty Vec<u64>, u64).
impl TryFrom<(Vec<u64>, u64)> for Noun {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, u64)) -> Result<Self, Self::Error> {
        Ok(Self::from(Cell::try_from((head, tail))?))
    }
}

/// Noun from (non-empty Vec<u64>, non-empty Vec<u64>).
impl TryFrom<(Vec<u64>, Vec<u64>)> for Noun {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self::from(Cell::try_from((head, tail))?))
    }
}

/// Noun from (non-empty Vec<u64>, Atom).
impl TryFrom<(Vec<u64>, Atom)> for Noun {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, Atom)) -> Result<Self, Self::Error> {
        Ok(Self::from(Cell::try_from((head, tail))?))
    }
}

/// Noun from (non-empty Vec<u64>, Cell).
impl TryFrom<(Vec<u64>, Cell)> for Noun {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, Cell)) -> Result<Self, Self::Error> {
        Ok(Self::from(Cell::try_from((head, tail))?))
    }
}

/// Noun from (non-empty Vec<u64>, Noun).
impl TryFrom<(Vec<u64>, Noun)> for Noun {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, Noun)) -> Result<Self, Self::Error> {
        Ok(Self::from(Cell::try_from((head, tail))?))
    }
}

/// Noun from (non-empty Vec<u64>, Rc<Noun>).
impl TryFrom<(Vec<u64>, Rc<Noun>)> for Noun {
    type Error = ();

    fn try_from((head, tail): (Vec<u64>, Rc<Noun>)) -> Result<Self, Self::Error> {
        Ok(Self::from(Cell::try_from((head, tail))?))
    }
}

/// Noun from (Atom, u64).
impl From<(Atom, u64)> for Noun {
    fn from((head, tail): (Atom, u64)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Atom, non-empty Vec<u64>).
impl TryFrom<(Atom, Vec<u64>)> for Noun {
    type Error = ();

    fn try_from((head, tail): (Atom, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self::from(Cell::try_from((head, tail))?))
    }
}

/// Noun from (Atom, Atom).
impl From<(Atom, Atom)> for Noun {
    fn from((head, tail): (Atom, Atom)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Atom, Cell).
impl From<(Atom, Cell)> for Noun {
    fn from((head, tail): (Atom, Cell)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Atom, Noun).
impl From<(Atom, Noun)> for Noun {
    fn from((head, tail): (Atom, Noun)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Atom, Rc<Noun>).
impl From<(Atom, Rc<Noun>)> for Noun {
    fn from((head, tail): (Atom, Rc<Noun>)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Cell, u64).
impl From<(Cell, u64)> for Noun {
    fn from((head, tail): (Cell, u64)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Cell, non-empty Vec<u64>).
impl TryFrom<(Cell, Vec<u64>)> for Noun {
    type Error = ();

    fn try_from((head, tail): (Cell, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self::from(Cell::try_from((head, tail))?))
    }
}

/// Noun from (Cell, Atom).
impl From<(Cell, Atom)> for Noun {
    fn from((head, tail): (Cell, Atom)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Cell, Cell).
impl From<(Cell, Cell)> for Noun {
    fn from((head, tail): (Cell, Cell)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Cell, Noun).
impl From<(Cell, Noun)> for Noun {
    fn from((head, tail): (Cell, Noun)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Cell, Rc<Noun>).
impl From<(Cell, Rc<Noun>)> for Noun {
    fn from((head, tail): (Cell, Rc<Noun>)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Noun, u64).
impl From<(Noun, u64)> for Noun {
    fn from((head, tail): (Noun, u64)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Noun, non-empty Vec<u64>).
impl TryFrom<(Noun, Vec<u64>)> for Noun {
    type Error = ();

    fn try_from((head, tail): (Noun, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self::from(Cell::try_from((head, tail))?))
    }
}

/// Noun from (Noun, Atom).
impl From<(Noun, Atom)> for Noun {
    fn from((head, tail): (Noun, Atom)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Noun, Cell).
impl From<(Noun, Cell)> for Noun {
    fn from((head, tail): (Noun, Cell)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Noun, Noun).
impl From<(Noun, Noun)> for Noun {
    fn from((head, tail): (Noun, Noun)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Noun, Rc<Noun>).
impl From<(Noun, Rc<Noun>)> for Noun {
    fn from((head, tail): (Noun, Rc<Noun>)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Rc<Noun>, u64).
impl From<(Rc<Noun>, u64)> for Noun {
    fn from((head, tail): (Rc<Noun>, u64)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Rc<Noun>, non-empty Vec<u64>).
impl TryFrom<(Rc<Noun>, Vec<u64>)> for Noun {
    type Error = ();

    fn try_from((head, tail): (Rc<Noun>, Vec<u64>)) -> Result<Self, Self::Error> {
        Ok(Self::from(Cell::try_from((head, tail))?))
    }
}

/// Noun from (Rc<Noun>, Atom).
impl From<(Rc<Noun>, Atom)> for Noun {
    fn from((head, tail): (Rc<Noun>, Atom)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Rc<Noun>, Cell).
impl From<(Rc<Noun>, Cell)> for Noun {
    fn from((head, tail): (Rc<Noun>, Cell)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Rc<Noun>, Noun).
impl From<(Rc<Noun>, Noun)> for Noun {
    fn from((head, tail): (Rc<Noun>, Noun)) -> Self {
        Self::from(Cell::from((head, tail)))
    }
}

/// Noun from (Rc<Noun>, Rc<Noun>).
impl From<(Rc<Noun>, Rc<Noun>)> for Noun {
    fn from((head, tail): (Rc<Noun>, Rc<Noun>)) -> Self {
        Self::from(Cell::from((head, tail)))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone() -> Result<(), ()> {
        // Clone 101010.
        {
            let n = Noun::from(101010);
            assert_eq!(n.clone(), n);
        }

        // Clone [300 [400 500]].
        {
            let n = Noun::from((300, Cell::from((400, 500))));
            assert_eq!(n.clone(), n);
        }

        Ok(())
    }

    #[test]
    fn partialeq() -> Result<(), ()> {
        // [0 5] == [0 5]
        {
            let lh = Noun::from((0, 5));
            let rh = Noun::from((0, 5));
            assert_eq!(lh, rh);
        }

        // [0 0] != [0 5]
        {
            let lh = Noun::from((0, 0));
            let rh = Noun::from((0, 5));
            assert_ne!(lh, rh);
        }

        // [[44 22] 88] == [[44 22] 88]
        {
            let lh = Noun::from((Cell::from((44, 22)), 88));
            let rh = Noun::from((Cell::from((44, 22)), 88));
            assert_eq!(lh, rh);
        }

        // [[44 22] 88] != [44 [22 88]]
        {
            let lh = Noun::from((Cell::from((44, 22)), 88));
            let rh = Noun::from((44, Cell::from((22, 88))));
            assert_ne!(lh, rh);
        }

        Ok(())
    }
}
