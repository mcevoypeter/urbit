use crate::{atom::Atom, cell::Cell};

#[macro_use]
pub mod atom;
pub mod cell;

pub trait Noun {
    fn is_atom(&self) -> bool;

    fn is_cell(&self) -> bool;

    fn as_atom(&self) -> Result<&Atom, ()>;

    fn as_cell(&self) -> Result<&Cell, ()>;
}
