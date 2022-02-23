use std::{error, fmt};

pub mod tree;

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

/// An atom or a cell.
///
/// This wraps the Atom and Cell structs to support functions that need to return a noun but don't
/// know whether that noun is an atom or a cell.
#[derive(Debug)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

impl Clone for Noun {
    fn clone(&self) -> Self {
        match self {
            Noun::Atom(a) => na!(a.0),
            Noun::Cell(c) => nc!(c.h.clone(), c.t.clone()),
        }
    }
}

impl PartialEq for Noun {
    fn eq(&self, other: &Self) -> bool {
        if let (Noun::Atom(lh), Noun::Atom(rh)) = (&*self, &*other) {
            lh == rh
        } else if let (Noun::Cell(lh), Noun::Cell(rh)) = (&*self, &*other) {
            lh == rh
        } else {
            false
        }
    }
}

impl fmt::Display for Noun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Noun::Atom(a) => {
                write!(f, "{}", a.0)
            }
            Noun::Cell(ref c) => {
                write!(f, "[{} {}]", c.h, c.t)
            }
        }
    }
}

impl Noun {
    pub fn new_atom(v: u64) -> Self {
        Noun::Atom(a!(v))
    }

    pub fn new_cell(h: Box<Self>, t: Box<Self>) -> Self {
        Noun::Cell(c!(h, t))
    }

    fn from_loobean(l: Loobean) -> Self {
        match l {
            Loobean::Yes => Noun::new_atom(0),
            Loobean::No => Noun::new_atom(1),
        }
    }

    pub fn into_box(self) -> Box<Self> {
        Box::new(self)
    }
}

/// Noun::new_atom($v)
#[macro_export]
macro_rules! na {
    ($v:expr) => {
        Noun::new_atom($v)
    };
}

/// Noun::new_cell($h, $t)
#[macro_export]
macro_rules! nc {
    ($h:expr, $t:expr) => {
        Noun::new_cell($h, $t)
    };
}

/// Box::new($e)
#[macro_export]
macro_rules! b {
    ($e:expr) => {
        Box::new($e)
    };
}

/// An unsigned integer.
#[derive(Clone, Debug, PartialEq)]
pub struct Atom(u64);

impl Atom {
    pub fn new(v: u64) -> Self {
        Atom(v)
    }
}

/// Atom::new($v)
#[macro_export]
macro_rules! a {
    ($v:expr) => {
        Atom::new($v)
    };
}

/// A pair of heap-allocated nouns.
#[derive(Debug)]
pub struct Cell {
    h: Box<Noun>,
    t: Box<Noun>,
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell {
            h: self.h.clone(),
            t: self.t.clone(),
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        if let (Noun::Atom(lh_h), Noun::Atom(rh_h)) = (&*self.h, &*other.h) {
            lh_h == rh_h && *self.t == *other.t
        } else if let (Noun::Cell(lh_h), Noun::Cell(rh_h)) = (&*self.h, &*other.h) {
            Self::eq(lh_h, rh_h) && *self.t == *other.t
        } else {
            false
        }
    }
}

impl Cell {
    pub fn new(h: Box<Noun>, t: Box<Noun>) -> Self {
        Cell { h, t }
    }

    pub fn head(&self) -> &Box<Noun> {
        &self.t
    }

    pub fn tail(&self) -> &Box<Noun> {
        &self.h
    }
}

/// Cell::new($h, $t)
#[macro_export]
macro_rules! c {
    ($h:expr, $t:expr) => {
        Cell::new($h, $t)
    };
}

/// A Nock-specific boolean where 0 is yes/true and 1 is no/false.
#[derive(Debug, PartialEq)]
pub enum Loobean {
    Yes,
    No,
}

impl Loobean {
    /// Convert a boolean into a Loobean.
    #[allow(dead_code)]
    fn into_boolean(self) -> bool {
        Loobean::Yes == self
    }

    /// Convert a Loobean into a boolean.
    fn from_boolean(b: bool) -> Self {
        if b {
            Loobean::Yes
        } else {
            Loobean::No
        }
    }
}

/// A Nock-specific error encapsulating an informative error message.
#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_atom() {
        // Clone 777.
        {
            let a = a!(777);
            assert_eq!(a, a.clone());
        }
    }

    #[test]
    fn clone_cell() {
        // Clone [8 808].
        {
            let c = c!(b!(na!(8)), b!(na!(808)));
            assert_eq!(c, c.clone());
        }
    }

    #[test]
    fn clone_noun() {
        // Clone 101010.
        {
            let noun = na!(101010);
            assert_eq!(noun, noun.clone());
        }

        // Clone [300 [400 500]].
        {
            let noun = nc!(b!(na!(300)), b!(nc!(b!(na!(400)), b!(na!(500)))));
            assert_eq!(noun, noun.clone());
        }
    }

    #[test]
    fn partialeq_cell() {
        // [71 109] == [71 109]
        {
            assert_eq!(c!(b!(na!(71)), b!(na!(109))), c!(b!(na!(71)), b!(na!(109))));
        }

        // [71 109] != [109 71]
        {
            assert_ne!(c!(b!(na!(71)), b!(na!(109))), c!(b!(na!(109)), b!(na!(71))));
        }

        // [11 [12 13]] == [11 [12 13]]
        {
            assert_eq!(
                nc!(b!(na!(11)), b!(nc!(b!(na!(12)), b!(na!(13))))),
                nc!(b!(na!(11)), b!(nc!(b!(na!(12)), b!(na!(13)))))
            );
        }

        // [11 [12 13]] != [11 [13 12]]
        {
            assert_ne!(
                nc!(b!(na!(11)), b!(nc!(b!(na!(12)), b!(na!(13))))),
                nc!(b!(na!(11)), b!(nc!(b!(na!(13)), b!(na!(12)))))
            );
        }
    }

    #[test]
    fn partialeq_noun() {
        // 500 == 500
        {
            assert_eq!(a!(500), a!(500));
        }

        // 499 != 501
        {
            assert_ne!(a!(499), a!(501));
        }

        // [0 5] == [0 5]
        {
            assert_eq!(nc!(b!(na!(0)), b!(na!(5))), nc!(b!(na!(0)), b!(na!(5))));
        }

        // [0 0] == [0 5]
        {
            assert_ne!(nc!(b!(na!(0)), b!(na!(0))), nc!(b!(na!(0)), b!(na!(5))));
        }

        // [[44 22] 88] == [[44 22] 88]
        {
            assert_eq!(
                nc!(b!(nc!(b!(na!(44)), b!(na!(22)))), b!(na!(88))),
                nc!(b!(nc!(b!(na!(44)), b!(na!(22)))), b!(na!(88)))
            );
        }

        // [[44 22] 88] != [44 [22 88]]
        {
            assert_ne!(
                nc!(b!(nc!(b!(na!(44)), b!(na!(22)))), b!(na!(88))),
                nc!(b!(na!(44)), b!(nc!(b!(na!(22)), b!(na!(88)))))
            );
        }
    }
}
