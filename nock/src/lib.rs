use crate::{atom::Atom, cell::Cell, error::Error, loobean::Loobean, noun::Noun};

pub mod atom;
pub mod cell;
pub mod error;
pub mod loobean;
pub mod noun;
pub mod interpreters;

/// The ? Nock operator.
///
/// Determine if a noun is a cell or an atom.
///
/// ```console
/// ?[x] -> 0 if x is a cell
/// ?[x] -> 1 if x is an atom
/// ```
trait Wut {
    fn wut(&self) -> Loobean;
}

/// The + Nock operator.
///
/// Increment an atom.
///
/// ```console
/// +[x] -> 1 + x   if x is an atom
/// +[x] -> *crash* if x is a cell
/// ```
trait Lus {
    fn lus(self) -> Atom;
}

/// The = Nock Operator.
///
/// Determine if two nouns are equal.
///
/// ```console
/// =[x y] -> 0 if x and y are the same noun
/// =[x y] -> 1 otherwise
/// ```
trait Tis {
    fn tis(&self) -> Loobean;
}

/// The / Nock operator.
trait Fas {
    fn fas(self) -> Result<Noun, Error>;
}

/// The # Nock operator.
trait Hax {
    fn hax(self) -> Result<Noun, Error>;
}

/// The * Nock operator.
///
/// Apply the Nock interpreter function.
pub trait Tar {
    fn tar(self) -> Result<Noun, Error>;
}

/// Box::new($e)
#[macro_export]
macro_rules! b {
    ($e:expr) => {
        Box::new($e)
    };
}
