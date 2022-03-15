use crate::{atom::Atom, cell::Cell};

#[macro_use]
pub mod atom;
#[macro_use]
pub mod cell;

pub trait Noun {
    fn is_atom(&self) -> bool;

    fn is_cell(&self) -> bool;

    fn as_atom(&self) -> Result<&Atom, Error>;

    fn as_cell(&self) -> Result<&Cell, Error>;

    fn into_atom(self) -> Result<Atom, Error>;

    fn into_cell(self) -> Result<Cell, Error>;
}

#[derive(Debug)]
pub enum Error {
    UnexpectedAtom,
    UnexpectedCell,
}
