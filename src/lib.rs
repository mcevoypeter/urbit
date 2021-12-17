#![allow(unused_parens)]

use std::{error, fmt};

/*==============================================================================
 * Struct and enum definitions
 */

#[derive(Debug, PartialEq)]
enum Loobean {
    Yes,
    No,
}

#[derive(Debug)]
struct Error {
    msg: String,
}

#[derive(Debug)]
enum Noun {
    Atom(Atom),
    Cell(Cell),
}

#[derive(Clone, Debug, PartialEq)]
struct Atom(u64);

#[derive(Debug)]
struct Cell {
    h: Box<Noun>,
    t: Box<Noun>,
}

/*==============================================================================
 * Trait definitions
 */

// ?
trait Wut {
    fn wut(&self) -> Loobean;
}

// +
trait Lus {
    fn lus(self) -> Atom;
}

// =
trait Tis {
    fn tis(&self) -> Loobean;
}

// /
trait Fas {
    fn fas(self) -> Result<Noun, Error>;
}

// #
trait Hax {
    fn hax(self) -> Result<Noun, Error>;
}

// *
trait Tar {
    fn tar(self) -> Result<Noun, Error>;
}

/*==============================================================================
 * Implementations for Loobean enum
 */

impl Loobean {
    #[allow(dead_code)]
    fn into_boolean(self) -> bool {
        Loobean::Yes == self
    }

    fn from_boolean(b: bool) -> Self {
        if b {
            Loobean::Yes
        } else {
            Loobean::No
        }
    }
}

/*==============================================================================
 * Implementations for Error struct
 */

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for Error {}

/*==============================================================================
 * Implementations for Noun enum
 */

impl Clone for Noun {
    fn clone(&self) -> Self {
        match self {
            Noun::Atom(a) => Noun::Atom(Atom(a.0)),
            Noun::Cell(c) => Noun::Cell(Cell {
                h: c.h.clone(),
                t: c.t.clone(),
            }),
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
    fn from_loobean(loob: Loobean) -> Self {
        match loob {
            Loobean::Yes => Atom(0).into_noun(),
            Loobean::No => Atom(1).into_noun(),
        }
    }

    fn into_box(self) -> Box<Self> {
        Box::new(self)
    }
}

/*==============================================================================
 * Implementations for Atom struct
 */

impl Wut for Atom {
    fn wut(&self) -> Loobean {
        Loobean::No
    }
}

impl Lus for Atom {
    fn lus(self) -> Atom {
        Atom(1 + self.0)
    }
}

impl Atom {
    fn into_noun(self) -> Noun {
        Noun::Atom(self)
    }
}

/*==============================================================================
 * Implementations for Cell struct
 */

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

impl Wut for Cell {
    fn wut(&self) -> Loobean {
        Loobean::Yes
    }
}

impl Tis for Cell {
    fn tis(&self) -> Loobean {
        Loobean::from_boolean(self.h == self.t)
    }
}

impl Fas for Cell {
    fn fas(self) -> Result<Noun, Error> {
        if let Noun::Atom(h) = *self.h {
            match h.0 {
                0 => Err(Error {
                    msg: "/[0 a] cannot be evaluated".to_string(),
                }),
                1 => Ok(*self.t),
                2 => {
                    if let Noun::Cell(t) = *self.t {
                        Ok(*t.h)
                    } else {
                        Err(Error {
                            msg: "/[2 a] cannot be evaluated when a is an atom".to_string(),
                        })
                    }
                }
                3 => {
                    if let Noun::Cell(t) = *self.t {
                        Ok(*t.t)
                    } else {
                        Err(Error {
                            msg: "/[3 a] cannot be evaluated when a is an atom".to_string(),
                        })
                    }
                }
                n => Cell {
                    h: Atom(2 + n % 2).into_noun().into_box(),
                    t: Cell {
                        h: Atom(n / 2).into_noun().into_box(),
                        t: self.t,
                    }
                    .fas()?
                    .into_box(),
                }
                .fas(),
            }
        } else {
            Err(Error {
                msg: "/[a b] cannot be evaluated when a is a cell".to_string(),
            })
        }
    }
}

impl Hax for Cell {
    fn hax(self) -> Result<Noun, Error> {
        if let (Noun::Atom(h), Noun::Cell(t)) = (*self.h, *self.t) {
            match h.0 {
                0 => Err(Error {
                    msg: "#[0 a b] cannot be evaluated".to_string(),
                }),
                1 => Ok(*t.h),
                n if 0 == n % 2 => Cell {
                    h: Atom(n / 2).into_noun().into_box(),
                    t: Cell {
                        h: Cell {
                            h: t.h,
                            t: Cell {
                                h: Atom(n + 1).into_noun().into_box(),
                                t: t.t.clone(),
                            }
                            .fas()?
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                        t: t.t,
                    }
                    .into_noun()
                    .into_box(),
                }
                .hax(),
                n => Cell {
                    h: Atom(n / 2).into_noun().into_box(),
                    t: Cell {
                        h: Cell {
                            h: Cell {
                                h: Atom(n - 1).into_noun().into_box(),
                                t: t.t.clone(),
                            }
                            .fas()?
                            .into_box(),
                            t: t.h,
                        }
                        .into_noun()
                        .into_box(),
                        t: t.t,
                    }
                    .into_noun()
                    .into_box(),
                }
                .hax(),
            }
        } else {
            Err(Error {
                msg: "#[a b] cannot be evaluated when a is cell and/or b is an atom".to_string(),
            })
        }
    }
}

impl Tar for Cell {
    fn tar(self) -> Result<Noun, Error> {
        if let Noun::Cell(t) = *self.t {
            if let Noun::Atom(Atom(opcode)) = *t.h {
                match opcode {
                    0 => Cell { h: t.t, t: self.h }.fas(),
                    1 => Ok(*t.t),
                    2 => {
                        if let Noun::Cell(tt) = *t.t {
                            Cell {
                                h: Cell {
                                    h: self.h.clone(),
                                    t: tt.h,
                                }
                                .tar()?
                                .into_box(),
                                t: Cell { h: self.h, t: tt.t }.tar()?.into_box(),
                            }
                            .tar()
                        } else {
                            Err(Error {
                                msg: "*[a 2 b] cannot be evaluated when b is an atom".to_string(),
                            })
                        }
                    }
                    3 => match (Cell { h: self.h, t: t.t }.tar()?) {
                        Noun::Atom(a) => Ok(Noun::from_loobean(a.wut())),
                        Noun::Cell(c) => Ok(Noun::from_loobean(c.wut())),
                    },
                    4 => {
                        if let Noun::Atom(a) = (Cell { h: self.h, t: t.t }.tar()?) {
                            Ok(a.lus().into_noun())
                        } else {
                            Err(Error {
                                msg: "Cannot apply the + operator to a cell".to_string(),
                            })
                        }
                    }
                    5 => {
                        if let Noun::Cell(tt) = *t.t {
                            Ok(Cell {
                                h: Cell {
                                    h: self.h.clone(),
                                    t: tt.h,
                                }
                                .tar()?
                                .into_box(),
                                t: Cell { h: self.h, t: tt.t }.tar()?.into_box(),
                            }
                            .into_noun())
                        } else {
                            Err(Error {
                                msg: "*[a 5 b] cannot be evaluated when b is an atom".to_string(),
                            })
                        }
                    }
                    6 => {
                        if let Noun::Cell(tt) = *t.t {
                            if let Noun::Cell(ttt) = *tt.t {
                                Cell {
                                    h: self.h.clone(),
                                    t: Cell {
                                        h: Cell { h: ttt.h, t: ttt.t }.into_noun().into_box(),
                                        t: Cell {
                                            h: Atom(0).into_noun().into_box(),
                                            t: Cell {
                                                h: Cell {
                                                    h: Atom(2).into_noun().into_box(),
                                                    t: Atom(3).into_noun().into_box(),
                                                }
                                                .into_noun()
                                                .into_box(),
                                                t: Cell {
                                                    h: Atom(0).into_noun().into_box(),
                                                    t: Cell {
                                                        h: self.h,
                                                        t: Cell {
                                                            h: Atom(4).into_noun().into_box(),
                                                            t: Cell {
                                                                h: Atom(4).into_noun().into_box(),
                                                                t: tt.h,
                                                            }
                                                            .into_noun()
                                                            .into_box(),
                                                        }
                                                        .into_noun()
                                                        .into_box(),
                                                    }
                                                    .tar()?
                                                    .into_box(),
                                                }
                                                .into_noun()
                                                .into_box(),
                                            }
                                            .tar()?
                                            .into_box(),
                                        }
                                        .into_noun()
                                        .into_box(),
                                    }
                                    .tar()?
                                    .into_box(),
                                }
                                .tar()
                            } else {
                                Err(Error {
                                    msg: "*[a 6 b c] cannot be evaluated when c is an atom"
                                        .to_string(),
                                })
                            }
                        } else {
                            Err(Error {
                                msg: "*[a 6 b] cannot be evaluated when b is an atom".to_string(),
                            })
                        }
                    }
                    7 => {
                        if let Noun::Cell(tt) = *t.t {
                            Cell {
                                h: Cell { h: self.h, t: tt.h }.tar()?.into_box(),
                                t: tt.t,
                            }
                            .tar()
                        } else {
                            Err(Error {
                                msg: "*[a 7 b] cannot be evaluated when b is an atom".to_string(),
                            })
                        }
                    }
                    8 => {
                        if let Noun::Cell(tt) = *t.t {
                            Cell {
                                h: Cell {
                                    h: Cell {
                                        h: self.h.clone(),
                                        t: tt.h,
                                    }
                                    .tar()?
                                    .into_box(),
                                    t: self.h,
                                }
                                .into_noun()
                                .into_box(),
                                t: tt.t,
                            }
                            .tar()
                        } else {
                            Err(Error {
                                msg: "*[a 8 b] cannot be evaluated when b is an atom".to_string(),
                            })
                        }
                    }
                    9 => {
                        if let Noun::Cell(tt) = *t.t {
                            Cell {
                                h: Cell { h: self.h, t: tt.t }.tar()?.into_box(),
                                t: Cell {
                                    h: Atom(2).into_noun().into_box(),
                                    t: Cell {
                                        h: Cell {
                                            h: Atom(0).into_noun().into_box(),
                                            t: Atom(1).into_noun().into_box(),
                                        }
                                        .into_noun()
                                        .into_box(),
                                        t: Cell {
                                            h: Atom(0).into_noun().into_box(),
                                            t: tt.h,
                                        }
                                        .into_noun()
                                        .into_box(),
                                    }
                                    .into_noun()
                                    .into_box(),
                                }
                                .into_noun()
                                .into_box(),
                            }
                            .tar()
                        } else {
                            Err(Error {
                                msg: "*[a 9 b] cannot be evaluated when b is an atom".to_string(),
                            })
                        }
                    }
                    10 => {
                        if let Noun::Cell(tt) = *t.t {
                            if let Noun::Cell(tth) = *tt.h {
                                Cell {
                                    h: tth.h,
                                    t: Cell {
                                        h: Cell {
                                            h: self.h.clone(),
                                            t: tth.t,
                                        }
                                        .tar()?
                                        .into_box(),
                                        t: Cell { h: self.h, t: tt.t }.tar()?.into_box(),
                                    }
                                    .into_noun()
                                    .into_box(),
                                }
                                .hax()
                            } else {
                                Err(Error {
                                    msg: "*[a 10 b c] cannot be evaluated when b is an atom"
                                        .to_string(),
                                })
                            }
                        } else {
                            Err(Error {
                                msg: "*[a 10 b] cannot be evaluated when b is an atom".to_string(),
                            })
                        }
                    }
                    11 => {
                        if let Noun::Cell(tt) = *t.t {
                            match *tt.h {
                                Noun::Atom(_) => Cell { h: self.h, t: tt.t }.tar(),
                                Noun::Cell(c) => Cell {
                                    h: Cell {
                                        h: Cell {
                                            h: self.h.clone(),
                                            t: c.t,
                                        }
                                        .tar()?
                                        .into_box(),
                                        t: Cell { h: self.h, t: tt.t }.tar()?.into_box(),
                                    }
                                    .into_noun()
                                    .into_box(),
                                    t: Cell {
                                        h: Atom(0).into_noun().into_box(),
                                        t: Atom(3).into_noun().into_box(),
                                    }
                                    .into_noun()
                                    .into_box(),
                                }
                                .tar(),
                            }
                        } else {
                            Err(Error {
                                msg: "*[a 11 b] cannot be evaluated when b is an atom".to_string(),
                            })
                        }
                    }
                    _ => Err(Error {
                        msg: "unsupported opcode".to_string(),
                    }),
                }
            } else {
                Err(Error {
                    msg: "*[a b c] cannot be evaluated when b is a cell".to_string(),
                })
            }
        } else {
            Err(Error {
                msg: "*[a b] cannot be evaluated when b is an atom".to_string(),
            })
        }
    }
}

impl Cell {
    fn into_noun(self) -> Noun {
        Noun::Cell(self)
    }
}

/*==============================================================================
 * Tests
 */

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn clone_atom() {
        // Clone 777.
        {
            let a = Atom(777);
            assert_eq!(a, a.clone());
        }
    }

    #[test]
    fn clone_cell() {
        // Clone [8 808].
        {
            let c = Cell {
                h: Atom(8).into_noun().into_box(),
                t: Atom(808).into_noun().into_box(),
            };
            assert_eq!(c, c.clone());
        }
    }

    #[test]
    fn clone_noun() {
        // Clone 101010.
        {
            let noun = Atom(101010).into_noun();
            assert_eq!(noun, noun.clone());
        }

        // Clone [300 [400 500]].
        {
            let noun = Cell {
                h: Atom(300).into_noun().into_box(),
                t: Cell {
                    h: Atom(400).into_noun().into_box(),
                    t: Atom(500).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .into_noun();
            assert_eq!(noun, noun.clone());
        }
    }

    #[test]
    fn decrement() {
        // [8 [1 0] 8 [1 6 [5 [0 7] 4 0 6] [0 6] 9 2 [0 2] [4 0 6] 0 7] 9 2 0 1]
        let decrement = Cell {
            h: Atom(8).into_noun().into_box(),
            t: Cell {
                h: Cell {
                    h: Atom(1).into_noun().into_box(),
                    t: Atom(0).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
                t: Cell {
                    h: Atom(8).into_noun().into_box(),
                    t: Cell {
                        h: Cell {
                            h: Atom(1).into_noun().into_box(),
                            t: Cell {
                                h: Atom(6).into_noun().into_box(),
                                t: Cell {
                                    h: Cell {
                                        h: Atom(5).into_noun().into_box(),
                                        t: Cell {
                                            h: Cell {
                                                h: Atom(0).into_noun().into_box(),
                                                t: Atom(7).into_noun().into_box(),
                                            }
                                            .into_noun()
                                            .into_box(),
                                            t: Cell {
                                                h: Atom(4).into_noun().into_box(),
                                                t: Cell {
                                                    h: Atom(0).into_noun().into_box(),
                                                    t: Atom(6).into_noun().into_box(),
                                                }
                                                .into_noun()
                                                .into_box(),
                                            }
                                            .into_noun()
                                            .into_box(),
                                        }
                                        .into_noun()
                                        .into_box(),
                                    }
                                    .into_noun()
                                    .into_box(),
                                    t: Cell {
                                        h: Cell {
                                            h: Atom(0).into_noun().into_box(),
                                            t: Atom(6).into_noun().into_box(),
                                        }
                                        .into_noun()
                                        .into_box(),
                                        t: Cell {
                                            h: Atom(9).into_noun().into_box(),
                                            t: Cell {
                                                h: Atom(2).into_noun().into_box(),
                                                t: Cell {
                                                    h: Cell {
                                                        h: Atom(0).into_noun().into_box(),
                                                        t: Atom(2).into_noun().into_box(),
                                                    }
                                                    .into_noun()
                                                    .into_box(),
                                                    t: Cell {
                                                        h: Cell {
                                                            h: Atom(4).into_noun().into_box(),
                                                            t: Cell {
                                                                h: Atom(0).into_noun().into_box(),
                                                                t: Atom(6).into_noun().into_box(),
                                                            }
                                                            .into_noun()
                                                            .into_box(),
                                                        }
                                                        .into_noun()
                                                        .into_box(),
                                                        t: Cell {
                                                            h: Atom(0).into_noun().into_box(),
                                                            t: Atom(7).into_noun().into_box(),
                                                        }
                                                        .into_noun()
                                                        .into_box(),
                                                    }
                                                    .into_noun()
                                                    .into_box(),
                                                }
                                                .into_noun()
                                                .into_box(),
                                            }
                                            .into_noun()
                                            .into_box(),
                                        }
                                        .into_noun()
                                        .into_box(),
                                    }
                                    .into_noun()
                                    .into_box(),
                                }
                                .into_noun()
                                .into_box(),
                            }
                            .into_noun()
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                        t: Cell {
                            h: Atom(9).into_noun().into_box(),
                            t: Cell {
                                h: Atom(2).into_noun().into_box(),
                                t: Cell {
                                    h: Atom(0).into_noun().into_box(),
                                    t: Atom(1).into_noun().into_box(),
                                }
                                .into_noun()
                                .into_box(),
                            }
                            .into_noun()
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .into_noun()
            .into_box(),
        }
        .into_noun()
        .into_box();

        // *[42 decrement] -> 41
        {
            match (Cell {
                h: Atom(42).into_noun().into_box(),
                t: decrement,
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Atom(41).into_noun(), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }
    }

    #[test]
    fn fas_cell() {
        // /[1 [98 89]] -> [98 89]
        {
            let t = Cell {
                h: Atom(98).into_noun().into_box(),
                t: Atom(89).into_noun().into_box(),
            }
            .into_noun()
            .into_box();
            match (Cell {
                h: Atom(1).into_noun().into_box(),
                t: t.clone(),
            }
            .fas())
            {
                Ok(res) => {
                    assert_eq!(*t, res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // /[2 [292 1001]] -> 292
        {
            let th = Atom(292).into_noun().into_box();
            match (Cell {
                h: Atom(2).into_noun().into_box(),
                t: Cell {
                    h: th.clone(),
                    t: Atom(1001).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .fas())
            {
                Ok(res) => {
                    assert_eq!(*th, res)
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // /[2 107] -> crash
        {
            match (Cell {
                h: Atom(2).into_noun().into_box(),
                t: Atom(107).into_noun().into_box(),
            }
            .fas())
            {
                Ok(_) => {
                    assert!(false, "Unexpected success.");
                }
                Err(_) => {
                    assert!(true);
                }
            }
        }

        // /[3 [[80 50] [19 95]]] -> [19 95]
        {
            let tt = Cell {
                h: Atom(19).into_noun().into_box(),
                t: Atom(95).into_noun().into_box(),
            }
            .into_noun()
            .into_box();
            match (Cell {
                h: Atom(3).into_noun().into_box(),
                t: Cell {
                    h: Cell {
                        h: Atom(80).into_noun().into_box(),
                        t: Atom(50).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: tt.clone(),
                }
                .into_noun()
                .into_box(),
            }
            .fas())
            {
                Ok(res) => {
                    assert_eq!(*tt, res)
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // /[5 [[15 16] 17]] -> 16
        {
            let tht = Atom(16).into_noun().into_box();
            match (Cell {
                h: Atom(5).into_noun().into_box(),
                t: Cell {
                    h: Cell {
                        h: Atom(15).into_noun().into_box(),
                        t: tht.clone(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Atom(17).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .fas())
            {
                Ok(res) => {
                    assert_eq!(*tht, res)
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // /[6 [4 [8 12]]] -> 8
        {
            let tth = Atom(8).into_noun().into_box();
            match (Cell {
                h: Atom(6).into_noun().into_box(),
                t: Cell {
                    h: Atom(4).into_noun().into_box(),
                    t: Cell {
                        h: tth.clone(),
                        t: Atom(12).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .fas())
            {
                Ok(res) => {
                    assert!(*tth == res)
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // /[12 [531 25 99]] -> crash
        {
            match (Cell {
                h: Atom(12).into_noun().into_box(),
                t: Cell {
                    h: Atom(531).into_noun().into_box(),
                    t: Cell {
                        h: Atom(25).into_noun().into_box(),
                        t: Atom(99).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .fas())
            {
                Ok(_) => {
                    assert!(false, "Unexpected success.")
                }
                Err(_) => {
                    assert!(true);
                }
            }
        }
    }

    #[test]
    fn hax_cell() {
        // #[1 [22 80]] -> 22
        {
            let th = Atom(22).into_noun().into_box();
            match (Cell {
                h: Atom(1).into_noun().into_box(),
                t: Cell {
                    h: th.clone(),
                    t: Atom(80).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .hax())
            {
                Ok(res) => {
                    assert_eq!(*th, res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // #[2 11 [22 33]] -> [11 33]
        {
            let th = Atom(11).into_noun().into_box();
            let ttt = Atom(33).into_noun().into_box();
            match (Cell {
                h: Atom(2).into_noun().into_box(),
                t: Cell {
                    h: th.clone(),
                    t: Cell {
                        h: Atom(22).into_noun().into_box(),
                        t: ttt.clone(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .hax())
            {
                Ok(res) => {
                    assert_eq!(Cell { h: th, t: ttt }.into_noun(), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // #[3 11 [22 33]] -> [22 11]
        {
            let th = Atom(11).into_noun().into_box();
            let tth = Atom(22).into_noun().into_box();
            match (Cell {
                h: Atom(3).into_noun().into_box(),
                t: Cell {
                    h: th.clone(),
                    t: Cell {
                        h: tth.clone(),
                        t: Atom(33).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .hax())
            {
                Ok(res) => {
                    assert_eq!(Cell { h: tth, t: th }.into_noun(), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // #[4 11 [[22 33] 44]] -> [[11 33] 44]
        {
            let th = Atom(11).into_noun().into_box();
            let ttht = Atom(33).into_noun().into_box();
            let ttt = Atom(44).into_noun().into_box();
            match (Cell {
                h: Atom(4).into_noun().into_box(),
                t: Cell {
                    h: th.clone(),
                    t: Cell {
                        h: Cell {
                            h: Atom(22).into_noun().into_box(),
                            t: ttht.clone(),
                        }
                        .into_noun()
                        .into_box(),
                        t: ttt.clone(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .hax())
            {
                Ok(res) => {
                    assert_eq!(
                        Cell {
                            h: Cell { h: th, t: ttht }.into_noun().into_box(),
                            t: ttt,
                        }
                        .into_noun(),
                        res
                    );
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // #[5 11 [[22 33] 44] -> [[22 11] 44]
        {
            let th = Atom(11).into_noun().into_box();
            let tthh = Atom(22).into_noun().into_box();
            let ttt = Atom(44).into_noun().into_box();
            match (Cell {
                h: Atom(5).into_noun().into_box(),
                t: Cell {
                    h: th.clone(),
                    t: Cell {
                        h: Cell {
                            h: tthh.clone(),
                            t: Atom(33).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                        t: ttt.clone(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .hax())
            {
                Ok(res) => {
                    assert_eq!(
                        Cell {
                            h: Cell { h: tthh, t: th }.into_noun().into_box(),
                            t: ttt,
                        }
                        .into_noun(),
                        res
                    );
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }
    }

    #[test]
    fn lus_atom() {
        // +999 -> 1000
        {
            let a = Atom(999);
            assert_eq!(1000, a.lus().0);
        }
    }

    #[test]
    fn partialeq_cell() {
        // [71 109] == [71 109]
        {
            assert_eq!(
                Cell {
                    h: Atom(71).into_noun().into_box(),
                    t: Atom(109).into_noun().into_box(),
                },
                Cell {
                    h: Atom(71).into_noun().into_box(),
                    t: Atom(109).into_noun().into_box(),
                },
            );
        }

        // [71 109] != [109 71]
        {
            assert_ne!(
                Cell {
                    h: Atom(71).into_noun().into_box(),
                    t: Atom(109).into_noun().into_box(),
                },
                Cell {
                    h: Atom(109).into_noun().into_box(),
                    t: Atom(71).into_noun().into_box(),
                },
            );
        }

        // [11 [12 13]] == [11 [12 13]]
        {
            assert_eq!(
                Cell {
                    h: Atom(11).into_noun().into_box(),
                    t: Cell {
                        h: Atom(12).into_noun().into_box(),
                        t: Atom(13).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                },
                Cell {
                    h: Atom(11).into_noun().into_box(),
                    t: Cell {
                        h: Atom(12).into_noun().into_box(),
                        t: Atom(13).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                },
            );
        }

        // [11 [12 13]] != [11 [13 12]]
        {
            assert_ne!(
                Cell {
                    h: Atom(11).into_noun().into_box(),
                    t: Cell {
                        h: Atom(12).into_noun().into_box(),
                        t: Atom(13).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                },
                Cell {
                    h: Atom(11).into_noun().into_box(),
                    t: Cell {
                        h: Atom(13).into_noun().into_box(),
                        t: Atom(12).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                },
            );
        }
    }

    #[test]
    fn partialeq_noun() {
        // 500 == 500
        {
            assert_eq!(Atom(500).into_noun(), Atom(500).into_noun(),);
        }

        // 499 != 501
        {
            assert_ne!(Atom(499).into_noun(), Atom(501).into_noun(),);
        }

        // [0 5] == [0 5]
        {
            assert_eq!(
                Cell {
                    h: Atom(0).into_noun().into_box(),
                    t: Atom(5).into_noun().into_box(),
                }
                .into_noun(),
                Cell {
                    h: Atom(0).into_noun().into_box(),
                    t: Atom(5).into_noun().into_box(),
                }
                .into_noun(),
            );
        }

        // [0 0] == [0 5]
        {
            assert_ne!(
                Cell {
                    h: Atom(0).into_noun().into_box(),
                    t: Atom(0).into_noun().into_box(),
                }
                .into_noun(),
                Cell {
                    h: Atom(0).into_noun().into_box(),
                    t: Atom(5).into_noun().into_box(),
                }
                .into_noun(),
            );
        }

        // [[44 22] 88] == [[44 22] 88]
        {
            assert_eq!(
                Cell {
                    h: Cell {
                        h: Atom(44).into_noun().into_box(),
                        t: Atom(22).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Atom(88).into_noun().into_box(),
                }
                .into_noun(),
                Cell {
                    h: Cell {
                        h: Atom(44).into_noun().into_box(),
                        t: Atom(22).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Atom(88).into_noun().into_box(),
                }
                .into_noun(),
            );
        }

        // [[44 22] 88] != [44 [22 88]]
        {
            assert_ne!(
                Cell {
                    h: Cell {
                        h: Atom(44).into_noun().into_box(),
                        t: Atom(22).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Atom(88).into_noun().into_box(),
                }
                .into_noun(),
                Cell {
                    h: Atom(44).into_noun().into_box(),
                    t: Cell {
                        h: Atom(22).into_noun().into_box(),
                        t: Atom(88).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun(),
            );
        }
    }

    #[test]
    fn tar_cell() {
        // *[1 0] -> crash
        {
            match (Cell {
                h: Atom(1).into_noun().into_box(),
                t: Atom(0).into_noun().into_box(),
            }
            .tar())
            {
                Ok(_) => {
                    assert!(false, "Unexpected success.");
                }
                Err(_) => {
                    assert!(true);
                }
            }
        }

        // *[4 [0 0] 4] -> crash
        {
            match (Cell {
                h: Atom(4).into_noun().into_box(),
                t: Cell {
                    h: Cell {
                        h: Atom(0).into_noun().into_box(),
                        t: Atom(0).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Atom(4).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(_) => {
                    assert!(false, "Unexpected success.");
                }
                Err(_) => {
                    assert!(true);
                }
            }
        }

        // *[[[4 5] [6 14 15]] [0 7]] -> [14 15]
        {
            let htt = Cell {
                h: Atom(14).into_noun().into_box(),
                t: Atom(15).into_noun().into_box(),
            }
            .into_noun()
            .into_box();
            match (Cell {
                h: Cell {
                    h: Cell {
                        h: Atom(4).into_noun().into_box(),
                        t: Atom(5).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Cell {
                        h: Atom(6).into_noun().into_box(),
                        t: htt.clone(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
                t: Cell {
                    h: Atom(0).into_noun().into_box(),
                    t: Atom(7).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(*htt, res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [1 153 218]] -> [153 218]
        {
            let tt = Cell {
                h: Atom(153).into_noun().into_box(),
                t: Atom(218).into_noun().into_box(),
            }
            .into_noun()
            .into_box();
            match (Cell {
                h: Atom(42).into_noun().into_box(),
                t: Cell {
                    h: Atom(1).into_noun().into_box(),
                    t: tt.clone(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(*tt, res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[77 [2 [1 42] [1 1 153 218]]] -> [153 218]
        {
            let ttttt = Cell {
                h: Atom(153).into_noun().into_box(),
                t: Atom(218).into_noun().into_box(),
            }
            .into_noun()
            .into_box();
            match (Cell {
                h: Atom(77).into_noun().into_box(),
                t: Cell {
                    h: Atom(2).into_noun().into_box(),
                    t: Cell {
                        h: Cell {
                            h: Atom(1).into_noun().into_box(),
                            t: Atom(42).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                        t: Cell {
                            h: Atom(1).into_noun().into_box(),
                            t: Cell {
                                h: Atom(1).into_noun().into_box(),
                                t: ttttt.clone(),
                            }
                            .into_noun()
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(*ttttt, res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[19 20] 3 0 1] -> 0
        {
            match (Cell {
                h: Cell {
                    h: Atom(19).into_noun().into_box(),
                    t: Atom(20).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
                t: Cell {
                    h: Atom(3).into_noun().into_box(),
                    t: Cell {
                        h: Atom(0).into_noun().into_box(),
                        t: Atom(1).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Atom(0).into_noun(), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[57 [4 0 1]] -> 58
        {
            match (Cell {
                h: Atom(57).into_noun().into_box(),
                t: Cell {
                    h: Atom(4).into_noun().into_box(),
                    t: Cell {
                        h: Atom(0).into_noun().into_box(),
                        t: Atom(1).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Atom(58).into_noun(), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[12 13] 5 [1 17] [0 3]] -> [17 13]
        {
            match (Cell {
                h: Cell {
                    h: Atom(12).into_noun().into_box(),
                    t: Atom(13).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
                t: Cell {
                    h: Atom(5).into_noun().into_box(),
                    t: Cell {
                        h: Cell {
                            h: Atom(1).into_noun().into_box(),
                            t: Atom(17).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                        t: Cell {
                            h: Atom(0).into_noun().into_box(),
                            t: Atom(3).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(
                        Cell {
                            h: Atom(17).into_noun().into_box(),
                            t: Atom(13).into_noun().into_box(),
                        }
                        .into_noun(),
                        res
                    );
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [6 [1 0] [4 0 1] [1 233]]] -> 43
        {
            match (Cell {
                h: Atom(42).into_noun().into_box(),
                t: Cell {
                    h: Atom(6).into_noun().into_box(),
                    t: Cell {
                        h: Cell {
                            h: Atom(1).into_noun().into_box(),
                            t: Atom(0).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                        t: Cell {
                            h: Cell {
                                h: Atom(4).into_noun().into_box(),
                                t: Cell {
                                    h: Atom(0).into_noun().into_box(),
                                    t: Atom(1).into_noun().into_box(),
                                }
                                .into_noun()
                                .into_box(),
                            }
                            .into_noun()
                            .into_box(),
                            t: Cell {
                                h: Atom(1).into_noun().into_box(),
                                t: Atom(233).into_noun().into_box(),
                            }
                            .into_noun()
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Atom(43).into_noun(), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [6 [1 1] [4 0 1] [1 233]]] -> 233
        {
            match (Cell {
                h: Atom(42).into_noun().into_box(),
                t: Cell {
                    h: Atom(6).into_noun().into_box(),
                    t: Cell {
                        h: Cell {
                            h: Atom(1).into_noun().into_box(),
                            t: Atom(1).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                        t: Cell {
                            h: Cell {
                                h: Atom(4).into_noun().into_box(),
                                t: Cell {
                                    h: Atom(0).into_noun().into_box(),
                                    t: Atom(1).into_noun().into_box(),
                                }
                                .into_noun()
                                .into_box(),
                            }
                            .into_noun()
                            .into_box(),
                            t: Cell {
                                h: Atom(1).into_noun().into_box(),
                                t: Atom(233).into_noun().into_box(),
                            }
                            .into_noun()
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Atom(233).into_noun(), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [7 [4 0 1] [4 0 1]]] -> 44
        {
            match (Cell {
                h: Atom(42).into_noun().into_box(),
                t: Cell {
                    h: Atom(7).into_noun().into_box(),
                    t: Cell {
                        h: Cell {
                            h: Atom(4).into_noun().into_box(),
                            t: Cell {
                                h: Atom(0).into_noun().into_box(),
                                t: Atom(1).into_noun().into_box(),
                            }
                            .into_noun()
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                        t: Cell {
                            h: Atom(4).into_noun().into_box(),
                            t: Cell {
                                h: Atom(0).into_noun().into_box(),
                                t: Atom(1).into_noun().into_box(),
                            }
                            .into_noun()
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Atom(44).into_noun(), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [8 [4 0 1] [0 1]]] -> [43 42]
        {
            match (Cell {
                h: Atom(42).into_noun().into_box(),
                t: Cell {
                    h: Atom(8).into_noun().into_box(),
                    t: Cell {
                        h: Cell {
                            h: Atom(4).into_noun().into_box(),
                            t: Cell {
                                h: Atom(0).into_noun().into_box(),
                                t: Atom(1).into_noun().into_box(),
                            }
                            .into_noun()
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                        t: Cell {
                            h: Atom(0).into_noun().into_box(),
                            t: Atom(1).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(
                        Cell {
                            h: Atom(43).into_noun().into_box(),
                            t: Atom(42).into_noun().into_box(),
                        }
                        .into_noun(),
                        res
                    );
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[422 [8 [4 0 1] [4 0 3]]] -> 43
        {
            match (Cell {
                h: Atom(42).into_noun().into_box(),
                t: Cell {
                    h: Atom(8).into_noun().into_box(),
                    t: Cell {
                        h: Cell {
                            h: Atom(4).into_noun().into_box(),
                            t: Cell {
                                h: Atom(0).into_noun().into_box(),
                                t: Atom(1).into_noun().into_box(),
                            }
                            .into_noun()
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                        t: Cell {
                            h: Atom(4).into_noun().into_box(),
                            t: Cell {
                                h: Atom(0).into_noun().into_box(),
                                t: Atom(3).into_noun().into_box(),
                            }
                            .into_noun()
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Atom(43).into_noun(), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[[0 1] 137] 9 2 [0 1]] -> [[0 1] 137]
        {
            match (Cell {
                h: Cell {
                    h: Cell {
                        h: Atom(0).into_noun().into_box(),
                        t: Atom(1).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Atom(137).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
                t: Cell {
                    h: Atom(9).into_noun().into_box(),
                    t: Cell {
                        h: Atom(2).into_noun().into_box(),
                        t: Cell {
                            h: Atom(0).into_noun().into_box(),
                            t: Atom(1).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(
                        Cell {
                            h: Cell {
                                h: Atom(0).into_noun().into_box(),
                                t: Atom(1).into_noun().into_box(),
                            }
                            .into_noun()
                            .into_box(),
                            t: Atom(137).into_noun().into_box(),
                        }
                        .into_noun(),
                        res
                    );
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[[0 2] 137] 9 2 [0 1]] -> [0 2]
        {
            match (Cell {
                h: Cell {
                    h: Cell {
                        h: Atom(0).into_noun().into_box(),
                        t: Atom(2).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Atom(137).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
                t: Cell {
                    h: Atom(9).into_noun().into_box(),
                    t: Cell {
                        h: Atom(2).into_noun().into_box(),
                        t: Cell {
                            h: Atom(0).into_noun().into_box(),
                            t: Atom(1).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(
                        Cell {
                            h: Atom(0).into_noun().into_box(),
                            t: Atom(2).into_noun().into_box(),
                        }
                        .into_noun(),
                        res
                    );
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[[0 2] 137] 9 2 [0 1]] -> [0 2]
        {
            match (Cell {
                h: Cell {
                    h: Cell {
                        h: Atom(0).into_noun().into_box(),
                        t: Atom(3).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Atom(137).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
                t: Cell {
                    h: Atom(9).into_noun().into_box(),
                    t: Cell {
                        h: Atom(2).into_noun().into_box(),
                        t: Cell {
                            h: Atom(0).into_noun().into_box(),
                            t: Atom(1).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Atom(137).into_noun(), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[16 32] 10 [1 0 2] 0 3] -> 16
        {
            match (Cell {
                h: Cell {
                    h: Atom(16).into_noun().into_box(),
                    t: Atom(32).into_noun().into_box(),
                }
                .into_noun()
                .into_box(),
                t: Cell {
                    h: Atom(10).into_noun().into_box(),
                    t: Cell {
                        h: Cell {
                            h: Atom(1).into_noun().into_box(),
                            t: Cell {
                                h: Atom(0).into_noun().into_box(),
                                t: Atom(2).into_noun().into_box(),
                            }
                            .into_noun()
                            .into_box(),
                        }
                        .into_noun()
                        .into_box(),
                        t: Cell {
                            h: Atom(0).into_noun().into_box(),
                            t: Atom(3).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .into_noun()
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Atom(16).into_noun(), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }
    }

    #[test]
    fn tis_cell() {
        // [2 2] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                Cell {
                    h: Atom(2).into_noun().into_box(),
                    t: Atom(2).into_noun().into_box(),
                }
                .tis(),
            );
        }

        // [7 6] -> 1
        {
            assert_eq!(
                Loobean::No,
                Cell {
                    h: Atom(7).into_noun().into_box(),
                    t: Atom(6).into_noun().into_box(),
                }
                .tis(),
            );
        }

        // [[2 7] [2 7]] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                Cell {
                    h: Cell {
                        h: Atom(2).into_noun().into_box(),
                        t: Atom(7).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Cell {
                        h: Atom(2).into_noun().into_box(),
                        t: Atom(7).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .tis(),
            );
        }

        // [[2 7] [2 [7 3]]] -> 1
        {
            assert_eq!(
                Loobean::No,
                Cell {
                    h: Cell {
                        h: Atom(2).into_noun().into_box(),
                        t: Atom(7).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Cell {
                        h: Atom(2).into_noun().into_box(),
                        t: Cell {
                            h: Atom(7).into_noun().into_box(),
                            t: Atom(3).into_noun().into_box(),
                        }
                        .into_noun()
                        .into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .tis(),
            );
        }
    }

    #[test]
    fn wut_atom() {
        // ?137 -> 1
        {
            assert_eq!(Loobean::No, Atom(137).wut(),);
        }
    }

    #[test]
    fn wut_cell() {
        // ?[128 256] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                Cell {
                    h: Atom(128).into_noun().into_box(),
                    t: Atom(256).into_noun().into_box(),
                }
                .wut(),
            );
        }

        // ?[[512 1024] [16 32]] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                Cell {
                    h: Cell {
                        h: Atom(512).into_noun().into_box(),
                        t: Atom(1024).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                    t: Cell {
                        h: Atom(16).into_noun().into_box(),
                        t: Atom(32).into_noun().into_box(),
                    }
                    .into_noun()
                    .into_box(),
                }
                .wut(),
            );
        }
    }
}
