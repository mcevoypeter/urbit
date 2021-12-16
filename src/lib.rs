use std::{error,fmt};

/*==============================================================================
 * Struct and enum definitions
 */

#[derive(Debug)]
#[derive(PartialEq)]
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

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
struct Atom(u64);

#[derive(Debug)]
struct Cell {
    head: Box<Noun>,
    tail: Box<Noun>,
}

/*==============================================================================
 * Macros
 */

#[macro_export]
macro_rules! cell {
    ($head:expr, $tail:expr,) => {
        Noun::Cell(Cell {
            head: $head,
            tail: $tail,
        })
    };
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
            Noun::Atom(atom) => {
                Noun::Atom(Atom(atom.0))
            },
            Noun::Cell(cell) => {
                cell! {
                    cell.head.clone(),
                    cell.tail.clone(),
                }
            },
        }
    }
}

impl PartialEq for Noun {
    fn eq(&self, other: &Self) -> bool {
        if let (Noun::Atom(lh), Noun::Atom(rh)) = (&*self, &*other) {
            lh == rh
        }
        else if let (Noun::Cell(lh), Noun::Cell(rh)) = (&*self, &*other) {
            lh == rh
        }
        else {
            false
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
            head: Box::new(*self.head.clone()),
            tail: Box::new(*self.tail.clone()),
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        if let (Noun::Atom(lh_head), Noun::Atom(rh_head))
            = (&*self.head, &*other.head)
        {
            lh_head == rh_head && *self.tail == *other.tail
        }
        else if let (Noun::Cell(lh_head), Noun::Cell(rh_head))
            = (&*self.head, &*other.head)
        {
            Self::eq(lh_head, rh_head) && *self.tail == *other.tail
        }
        else {
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
        if self.head == self.tail {
            Loobean::Yes
        } else {
            Loobean::No
        }
    }
}

impl Fas for Cell {
    fn fas(self) -> Result<Noun, Error> {
        if let Noun::Atom(head) = *self.head {
            match head {
                Atom(0) => Err(Error {
                    msg: "/[0 b] cannot be evaluated".to_string(),
                }),
                Atom(1) => Ok(*self.tail),
                Atom(n) => {
                    // XXX: is it true that b has to be a cell for /[(a + a) b] and
                    // /[(a + a + 1) b]?
                    if let Noun::Cell(tail) = *self.tail {
                        match n {
                            2 => Ok(*tail.head),
                            3 => Ok(*tail.tail),
                            _ => {
                                Cell {
                                    head: Atom(2 + n % 2).into_noun().into_box(),
                                    tail: Box::new(Cell {
                                        head: Atom(n / 2).into_noun().into_box(),
                                        tail: Box::new(Noun::Cell(tail)),
                                    }.fas()?),
                                }.fas()
                            },
                        }
                    }
                    else {
                        Err(Error {
                            msg: "/[a b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                },
            }
        }
        else {
            Err(Error {
                msg: "/[a b] cannot be evaluated when a is a cell".to_string(),
            })
        }
    }
}

impl Hax for Cell {
    fn hax(self) -> Result<Noun, Error> {
        if let (Noun::Atom(head), Noun::Cell(tail)) = (*self.head, *self.tail) {
            match head {
                Atom(0) => Err(Error {
                    msg: "#[0 a b] cannot be evaluated".to_string(),
                }),
                Atom(1) => Ok(*tail.head),
                Atom(n) if 0 == n % 2 => {
                    Cell {
                        head: Atom(n / 2).into_noun().into_box(),
                        tail: Box::new(cell! {
                            Box::new(cell! {
                                Box::new(*tail.head),
                                Box::new(Cell {
                                    head: Atom(n + 1).into_noun().into_box(),
                                    tail: Box::new(*tail.tail.clone()),
                                }.fas()?),
                            }),
                            Box::new(*tail.tail),
                        }),
                    }.hax()
                },
                Atom(n) => {
                    Cell {
                        head: Atom(n / 2).into_noun().into_box(),
                        tail: Box::new(cell! {
                            Box::new(cell! {
                                Box::new(Cell {
                                    head: Atom(n - 1).into_noun().into_box(),
                                    tail: Box::new(*tail.tail.clone()),
                                }.fas()?),
                                Box::new(*tail.head),
                            }),
                            Box::new(*tail.tail),
                        }),
                    }.hax()
                },
            }
        }
        else {
            Err(Error {
                msg: "#[a b] cannot be evaluated when a is cell and/or b is an atom".to_string(),
            })
        }
    }
}

impl Tar for Cell {
    fn tar(self) -> Result<Noun, Error> {
        if let Noun::Cell(tail) = *self.tail {
            if let Noun::Atom(opcode) = *tail.head {
                match opcode {
                    Atom(0) => Cell {
                        head: tail.tail,
                        tail: self.head,
                    }.fas(),
                    Atom(1) => Ok(*tail.tail),
                    Atom(2) => {
                        match *tail.tail {
                            Noun::Atom(_) => Err(Error {
                                msg: "*[a 2 b] cannot be evaluated when b is an atom".to_string(),
                            }),
                            Noun::Cell(tail_tail) => Cell {
                                head: Box::new(Cell {
                                    head: self.head.clone(),
                                    tail: tail_tail.head,
                                }.tar()?),
                                tail: Box::new(Cell {
                                    head: self.head,
                                    tail: tail_tail.tail,
                                }.tar()?),
                            }.tar(),
                        }
                    },
                    Atom(3) => {
                        match (Cell {
                            head: self.head,
                            tail: tail.tail,
                        }.tar()?)
                        {
                            Noun::Atom(atom) => Ok(Noun::from_loobean(atom.wut())),
                            Noun::Cell(cell) => Ok(Noun::from_loobean(cell.wut())),
                        }
                    },
                    Atom(4) => {
                        match (Cell {
                            head: self.head,
                            tail: tail.tail,
                        }.tar()?)
                        {
                            Noun::Atom(atom) => Ok(Noun::Atom(atom.lus())),
                            Noun::Cell(_) => Err(Error {
                                msg: "Cannot apply the + operator to a cell".to_string(),
                            }),
                        }

                    },
                    Atom(5) => {
                        match *tail.tail {
                            Noun::Atom(_) => Err(Error {
                                msg: "*[a 5 b] cannot be evaluated when b is an atom".to_string(),
                            }),
                            Noun::Cell(tail_tail) => Ok(cell! {
                                Box::new(Cell {
                                    head: self.head.clone(),
                                    tail: tail_tail.head,
                                }.tar()?),
                                Box::new(Cell {
                                    head: self.head,
                                    tail: tail_tail.tail,
                                }.tar()?),
                            }),
                        }
                    },
                    Atom(6) => Err(Error {
                        msg: "unimplemented".to_string(),
                    }),
                    Atom(7) => Err(Error {
                        msg: "unimplemented".to_string(),
                    }),
                    Atom(8) => Err(Error {
                        msg: "unimplemented".to_string(),
                    }),
                    Atom(9) => Err(Error {
                        msg: "unimplemented".to_string(),
                    }),
                    Atom(10) => Err(Error {
                        msg: "unimplemented".to_string(),
                    }),
                    Atom(11) => Err(Error {
                        msg: "unimplemented".to_string(),
                    }),
                    _ => Err(Error {
                        msg: "unsupported opcode".to_string(),
                    }),
                }
            }
            else {
                Err(Error {
                    msg: "*[a b c] cannot be evaluated when b is a cell".to_string(),
                })
            }
        }
        else {
            Err(Error {
                msg: "*[a b] cannot be evaluated when b is an atom".to_string(),
            })
        }
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
            let atom = Atom(777);
            assert_eq!(atom, atom.clone());
        }
    }

    #[test]
    fn clone_cell() {
        // Clone [8 808].
        {
            let cell = Cell {
                head: Atom(8).into_noun().into_box(),
                tail: Atom(808).into_noun().into_box(),
            };
            assert_eq!(cell, cell.clone());
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
            let noun = cell! {
                Atom(300).into_noun().into_box(),
                Box::new(cell! {
                    Atom(400).into_noun().into_box(),
                    Atom(500).into_noun().into_box(),
                }),
            };
            assert_eq!(noun, noun.clone());
        }
    }

    #[test]
    fn fas_cell() {
        // /[1 [98 89]] -> [98 89]
        {
            let t = Box::new(cell! {
                Atom(98).into_noun().into_box(),
                Atom(89).into_noun().into_box(),
            });
            match (Cell {
                head: Atom(1).into_noun().into_box(),
                tail: t.clone(),
            }.fas())
            {
                Ok(res) => {
                    assert_eq!(*t, res);
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // /[2 [292 1001]] -> 292
        {
            let th = Atom(292).into_noun().into_box();
            match (Cell {
                head: Atom(2).into_noun().into_box(),
                tail: Box::new(cell! {
                    th.clone(),
                    Atom(1001).into_noun().into_box(),
                }),
            }.fas())
            {
                Ok(res) => {
                    assert_eq!(*th, res)
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // /[2 107] -> crash
        {
            match (Cell {
                head: Atom(2).into_noun().into_box(),
                tail: Atom(107).into_noun().into_box(),
            }.fas())
            {
                Ok(_) => {
                    assert!(false, "Unexpected success.");
                },
                Err(_) => {
                    assert!(true);
                },
            }
        }

        // /[3 [[80 50] [19 95]]] -> [19 95]
        {
            let tt = Box::new(cell! {
                Atom(19).into_noun().into_box(),
                Atom(95).into_noun().into_box(),
            });
            match (Cell {
                head: Atom(3).into_noun().into_box(),
                tail: Box::new(cell! {
                    Box::new(cell! {
                        Atom(80).into_noun().into_box(),
                        Atom(50).into_noun().into_box(),
                    }),
                    tt.clone(),
                }),
            }.fas())
            {
                Ok(res) => {
                    assert_eq!(*tt, res)
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // /[5 [[15 16] 17]] -> 16
        {
            let tht = Atom(16).into_noun().into_box();
            match (Cell {
                head: Atom(5).into_noun().into_box(),
                tail: Box::new(cell! {
                    Box::new(cell! {
                        Atom(15).into_noun().into_box(),
                        tht.clone(),
                    }),
                    Atom(17).into_noun().into_box(),
                }),
            }.fas())
            {
                Ok(res) => {
                    assert_eq!(*tht, res)
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // /[6 [4 [8 12]]] -> 8
        {
            let tth = Atom(8).into_noun().into_box();
            match (Cell {
                head: Atom(6).into_noun().into_box(),
                tail: Box::new(cell! {
                    Atom(4).into_noun().into_box(),
                    Box::new(cell! {
                        tth.clone(),
                        Atom(12).into_noun().into_box(),
                    }),
                }),
            }.fas())
            {
                Ok(res) => {
                    assert!(*tth == res)
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // /[12 [531 25 99]] -> crash
        {
            match (Cell {
                head: Atom(12).into_noun().into_box(),
                tail: Box::new(cell! {
                    Atom(531).into_noun().into_box(),
                    Box::new(cell! {
                        Atom(25).into_noun().into_box(),
                        Atom(99).into_noun().into_box(),
                    }),
                }),
            }.fas())
            {
                Ok(_) => {
                    assert!(false, "Unexpected success.")
                },
                Err(_) => {
                    assert!(true);
                },
            }
        }
    }

    #[test]
    fn hax_cell() {
        // #[1 [22 80]] -> 22
        {
            let th = Atom(22).into_noun().into_box();
            match (Cell {
                head: Atom(1).into_noun().into_box(),
                tail: Box::new(cell! {
                    th.clone(),
                    Atom(80).into_noun().into_box(),
                }),
            }.hax())
            {
                Ok(res) => {
                    assert_eq!(*th, res);
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // #[2 11 [22 33]] -> [11 33]
        {
            let th = Atom(11).into_noun().into_box();
            let ttt = Atom(33).into_noun().into_box();
            match (Cell {
                head: Atom(2).into_noun().into_box(),
                tail: Box::new(cell! {
                    th.clone(),
                    Box::new(cell! {
                        Atom(22).into_noun().into_box(),
                        ttt.clone(),
                    }),
                }),
            }.hax())
            {
                Ok(res) => {
                    assert_eq!(
                        cell!{ th, ttt, },
                        res
                    );
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // #[3 11 [22 33]] -> [22 11]
        {
            let th = Atom(11).into_noun().into_box();
            let tth = Atom(22).into_noun().into_box();
            match (Cell {
                head: Atom(3).into_noun().into_box(),
                tail: Box::new(cell! {
                    th.clone(),
                    Box::new(cell! {
                        tth.clone(),
                        Atom(33).into_noun().into_box(),
                    }),
                }),
            }.hax())
            {
                Ok(res) => {
                    assert_eq!(
                        cell!{ tth, th, },
                        res
                        );
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // #[4 11 [[22 33] 44]] -> [[11 33] 44]
        {
            let th = Atom(11).into_noun().into_box();
            let ttht = Atom(33).into_noun().into_box();
            let ttt = Atom(44).into_noun().into_box();
            match (Cell {
                head: Atom(4).into_noun().into_box(),
                tail: Box::new(cell! {
                    th.clone(),
                    Box::new(cell! {
                        Box::new(cell! {
                            Atom(22).into_noun().into_box(),
                            ttht.clone(),
                        }),
                        ttt.clone(),
                    }),
                }),
            }.hax())
            {
                Ok(res) => {
                    assert_eq!(
                        cell!{
                            Box::new(cell!{
                                th,
                                ttht,
                            }),
                            ttt,
                        },
                        res
                        );
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // #[5 11 [[22 33] 44] -> [[22 11] 44]
        {
            let th = Atom(11).into_noun().into_box();
            let tthh = Atom(22).into_noun().into_box();
            let ttt = Atom(44).into_noun().into_box();
            match (Cell {
                head: Atom(5).into_noun().into_box(),
                tail: Box::new(cell! {
                    th.clone(),
                    Box::new(cell! {
                        Box::new(cell! {
                            tthh.clone(),
                            Atom(33).into_noun().into_box(),
                        }),
                        ttt.clone(),
                    }),
                }),
            }.hax())
            {
                Ok(res) => {
                    assert_eq!(
                        cell!{
                            Box::new(cell!{
                                tthh,
                                th,
                            }),
                            ttt,
                        },
                        res
                        );
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }
    }

    #[test]
    fn lus_atom() {
        // +999 -> 1000
        {
            let atom = Atom(999);
            assert_eq!(1000, atom.lus().0);
        }
    }

    #[test]
    fn partialeq_cell() {
        // [71 109] == [71 109]
        {
            assert_eq!(
                Cell {
                    head: Atom(71).into_noun().into_box(),
                    tail: Atom(109).into_noun().into_box(),
                },
                Cell {
                    head: Atom(71).into_noun().into_box(),
                    tail: Atom(109).into_noun().into_box(),
                },
            );
        }

        // [71 109] != [109 71]
        {
            assert_ne!(
                Cell {
                    head: Atom(71).into_noun().into_box(),
                    tail: Atom(109).into_noun().into_box(),
                },
                Cell {
                    head: Atom(109).into_noun().into_box(),
                    tail: Atom(71).into_noun().into_box(),
                },
            );
        }

        // [11 [12 13]] == [11 [12 13]]
        {
            assert_eq!(
                Cell {
                    head: Atom(11).into_noun().into_box(),
                    tail: Box::new(cell! {
                        Atom(12).into_noun().into_box(),
                        Atom(13).into_noun().into_box(),
                    }),
                },
                Cell {
                    head: Atom(11).into_noun().into_box(),
                    tail: Box::new(cell! {
                        Atom(12).into_noun().into_box(),
                        Atom(13).into_noun().into_box(),
                    }),
                },
            );
        }

        // [11 [12 13]] != [11 [13 12]]
        {
            assert_ne!(
                Cell {
                    head: Atom(11).into_noun().into_box(),
                    tail: Box::new(cell! {
                        Atom(12).into_noun().into_box(),
                        Atom(13).into_noun().into_box(),
                    }),
                },
                Cell {
                    head: Atom(11).into_noun().into_box(),
                    tail: Box::new(cell! {
                        Atom(13).into_noun().into_box(),
                        Atom(12).into_noun().into_box(),
                    }),
                },
            );
        }
    }

    #[test]
    fn partialeq_noun() {
        // 500 == 500
        {
            assert_eq!(
                Atom(500).into_noun(),
                Atom(500).into_noun(),
            );
        }

        // 499 != 501
        {
            assert_ne!(
                Atom(499).into_noun(),
                Atom(501).into_noun(),
            );
        }

        // [0 5] == [0 5]
        {
            assert_eq!(
                cell! {
                    Atom(0).into_noun().into_box(),
                    Atom(5).into_noun().into_box(),
                },
                cell! {
                    Atom(0).into_noun().into_box(),
                    Atom(5).into_noun().into_box(),
                },
            );
        }

        // [0 0] == [0 5]
        {
            assert_ne!(
                cell! {
                    Atom(0).into_noun().into_box(),
                    Atom(0).into_noun().into_box(),
                },
                cell! {
                    Atom(0).into_noun().into_box(),
                    Atom(5).into_noun().into_box(),
                },
            );
        }

        // [[44 22] 88] == [[44 22] 88]
        {
            assert_eq!(
                cell! {
                    Box::new(cell! {
                        Atom(44).into_noun().into_box(),
                        Atom(22).into_noun().into_box(),
                    }),
                    Atom(88).into_noun().into_box(),
                },
                cell! {
                    Box::new(cell! {
                        Atom(44).into_noun().into_box(),
                        Atom(22).into_noun().into_box(),
                    }),
                    Atom(88).into_noun().into_box(),
                },
            );
        }

        // [[44 22] 88] != [44 [22 88]]
        {
            assert_ne!(
                cell! {
                    Box::new(cell! {
                        Atom(44).into_noun().into_box(),
                        Atom(22).into_noun().into_box(),
                    }),
                    Atom(88).into_noun().into_box(),
                },
                cell! {
                    Atom(44).into_noun().into_box(),
                    Box::new(cell! {
                        Atom(22).into_noun().into_box(),
                        Atom(88).into_noun().into_box(),
                    }),
                },
            );
        }
    }

    #[test]
    fn tar_cell() {
        // *[1 0] -> crash
        {
            match (Cell {
                head: Atom(1).into_noun().into_box(),
                tail: Atom(0).into_noun().into_box(),
            }.tar())
            {
                Ok(_) => {
                    assert!(false, "Unexpected success.");
                },
                Err(_) => {
                    assert!(true);
                },
            }
        }

        // *[4 [0 0] 4] -> crash
        {
            match (Cell {
                head: Atom(4).into_noun().into_box(),
                tail: Box::new(cell! {
                    Box::new(cell! {
                        Atom(0).into_noun().into_box(),
                        Atom(0).into_noun().into_box(),
                    }),
                    Atom(4).into_noun().into_box(),
                }),
            }.tar())
            {
                Ok(_) => {
                    assert!(false, "Unexpected success.");
                },
                Err(_) => {
                    assert!(true);
                },
            }
        }

        // *[[[4 5] [6 14 15]] [0 7]] -> [14 15]
        {
            let htt = Box::new(cell! {
                Atom(14).into_noun().into_box(),
                Atom(15).into_noun().into_box(),
            });
            match (Cell {
                head: Box::new(cell! {
                    Box::new(cell! {
                        Atom(4).into_noun().into_box(),
                        Atom(5).into_noun().into_box(),
                    }),
                    Box::new(cell! {
                        Atom(6).into_noun().into_box(),
                        htt.clone(),
                    }),
                }),
                tail: Box::new(cell! {
                    Atom(0).into_noun().into_box(),
                    Atom(7).into_noun().into_box(),
                }),
            }.tar())
            {
                Ok(res) => {
                    assert_eq!(*htt, res);
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // *[42 [1 153 218]] -> [153 218]
        {
            let tt = Box::new(cell! {
                Atom(153).into_noun().into_box(),
                Atom(218).into_noun().into_box(),
            });
            match (Cell {
                head: Atom(42).into_noun().into_box(),
                tail: Box::new(cell!{
                    Atom(1).into_noun().into_box(),
                    tt.clone(),
                }),
            }.tar())
            {
                Ok(res) => {
                    assert_eq!(*tt, res);
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // *[77 [2 [1 42] [1 1 153 218]]] -> [153 218]
        {
            let ttttt = Box::new(cell! {
                Atom(153).into_noun().into_box(),
                Atom(218).into_noun().into_box(),
            });
            match (Cell {
                head: Atom(77).into_noun().into_box(),
                tail:Box::new(cell! {
                    Atom(2).into_noun().into_box(),
                    Box::new(cell! {
                        Box::new(cell! {
                            Atom(1).into_noun().into_box(),
                            Atom(42).into_noun().into_box(),
                        }),
                        Box::new(cell! {
                            Atom(1).into_noun().into_box(),
                            Box::new(cell! {
                                Atom(1).into_noun().into_box(),
                                ttttt.clone(),
                            }),
                        }),
                    }),
                }),
            }.tar())
            {
                Ok(res) => {
                    assert_eq!(*ttttt, res);
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // *[[19 20] 3 0 1] -> 0
        {
            match (Cell {
                head: Box::new(cell! {
                    Atom(19).into_noun().into_box(),
                    Atom(20).into_noun().into_box(),
                }),
                tail: Box::new(cell! {
                    Atom(3).into_noun().into_box(),
                    Box::new(cell! {
                        Atom(0).into_noun().into_box(),
                        Atom(1).into_noun().into_box(),
                    }),
                }),
            }.tar())
            {
                Ok(res) => {
                    assert_eq!(Atom(0).into_noun(), res);
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // *[57 [4 0 1]] -> 58
        {
            match (Cell {
                head: Atom(57).into_noun().into_box(),
                tail: Box::new(cell! {
                    Atom(4).into_noun().into_box(),
                    Box::new(cell!{
                        Atom(0).into_noun().into_box(),
                        Atom(1).into_noun().into_box(),
                    }),
                }),
            }.tar())
            {
                Ok(res) => {
                    assert_eq!(Atom(58).into_noun(), res);
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
            }
        }

        // *[[12 13] 5 [1 17] [0 3]] -> [17 13]
        {
            match (Cell {
                head: Box::new(cell! {
                    Atom(12).into_noun().into_box(),
                    Atom(13).into_noun().into_box(),
                }),
                tail: Box::new(cell! {
                    Atom(5).into_noun().into_box(),
                    Box::new(cell! {
                        Box::new(cell! {
                            Atom(1).into_noun().into_box(),
                            Atom(17).into_noun().into_box(),
                        }),
                        Box::new(cell! {
                            Atom(0).into_noun().into_box(),
                            Atom(3).into_noun().into_box(),
                        }),
                    }),
                }),
            }.tar())
            {
                Ok(res) => {
                    assert_eq!(
                        cell! {
                            Atom(17).into_noun().into_box(),
                            Atom(13).into_noun().into_box(),
                        },
                        res
                    );
                },
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                },
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
                    head: Atom(2).into_noun().into_box(),
                    tail: Atom(2).into_noun().into_box(),
                }.tis(),
            );
        }

        // [7 6] -> 1
        {
            assert_eq!(
                Loobean::No,
                Cell {
                    head: Atom(7).into_noun().into_box(),
                    tail: Atom(6).into_noun().into_box(),
                }.tis(),
            );
        }

        // [[2 7] [2 7]] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                Cell {
                    head: Box::new(cell! {
                        Atom(2).into_noun().into_box(),
                        Atom(7).into_noun().into_box(),
                    }),
                    tail: Box::new(cell! {
                        Atom(2).into_noun().into_box(),
                        Atom(7).into_noun().into_box(),
                    }),
                }.tis(),
            );
        }

        // [[2 7] [2 [7 3]]] -> 1
        {
            assert_eq!(
                Loobean::No,
                Cell {
                    head: Box::new(cell! {
                        Atom(2).into_noun().into_box(),
                        Atom(7).into_noun().into_box(),
                    }),
                    tail: Box::new(cell! {
                        Atom(2).into_noun().into_box(),
                        Box::new(cell! {
                            Atom(7).into_noun().into_box(),
                            Atom(3).into_noun().into_box(),
                        }),
                    }),
                }.tis(),
            );
        }
    }

    #[test]
    fn wut_atom() {
        // ?137 -> 1
        {
            assert_eq!(
                Loobean::No,
                Atom(137).wut(),
            );
        }
    }

    #[test]
    fn wut_cell() {
        // ?[128 256] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                Cell {
                    head: Atom(128).into_noun().into_box(),
                    tail: Atom(256).into_noun().into_box(),
                }.wut(),
            );
        }

        // ?[[512 1024] [16 32]] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                Cell {
                    head: Box::new(cell! {
                        Atom(512).into_noun().into_box(),
                        Atom(1024).into_noun().into_box(),
                    }),
                    tail: Box::new(cell! {
                        Atom(16).into_noun().into_box(),
                        Atom(32).into_noun().into_box(),
                    }),
                }.wut(),
            );
        }
    }
}
