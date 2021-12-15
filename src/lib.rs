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
macro_rules! atom {
    ($val:expr) => {
        Noun::Atom(Atom($val))
    };
}

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
    fn hax(&self) {}
}

// *
trait Tar {
    fn tar(&self) {}
}

impl fmt::Display for Error {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}", self.msg)
   }
}

/*==============================================================================
 * Trait implementations for Error struct
 */

impl error::Error for Error {}

impl Clone for Noun {
    fn clone(&self) -> Self {
        match self {
            Noun::Atom(atom) => {
                atom!(atom.0)
            },
            Noun::Cell(cell) => {
                cell!(
                    cell.head.clone(),
                    cell.tail.clone(),
                )
            },
        }
    }
}

impl PartialEq for Noun {
    fn eq(&self, other: &Self) -> bool {
        if let (Noun::Atom(lh), Noun::Atom(rh)) = (&*self, &*other) {
            return lh == rh
        }

        if let (Noun::Cell(lh), Noun::Cell(rh)) = (&*self, &*other) {
            return lh == rh
        }

        false
    }
}

/*==============================================================================
 * Trait implementations for Atom struct
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

/*==============================================================================
 * Trait implementations for Cell struct
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

        // The heads of both self and other are atoms.
        if let (Noun::Atom(lh_head), Noun::Atom(rh_head))
            = (&*self.head, &*other.head)
        {
            if lh_head != rh_head {
                return false
            }
            return *self.tail == *other.tail
        }

        // The heads of both self and other are cells.
        if let (Noun::Cell(lh_head), Noun::Cell(rh_head))
            = (&*self.head, &*other.head)
        {
            if !Self::eq(lh_head, rh_head) {
                return false
            }

            return *self.tail == *other.tail
        }

        false
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
                Atom(1) => Ok(*self.tail),
                Atom(2) => {
                    if let Noun::Cell(cell) = *self.tail {
                        Ok(*cell.head)
                    } else {
                        Err(Error {
                            msg: "/[2 a] cannot be evaluated when a is an atom".to_string(),
                        })
                    }
                },
                Atom(3) => {
                    if let Noun::Cell(cell) = *self.tail {
                        Ok(*cell.tail)
                    } else {
                        Err(Error {
                            msg: "/[3 a] cannot be evaluated when a is an atom".to_string(),
                        })
                    }
                },
                Atom(n) => {
                    if let Noun::Cell(_) = *self.tail {
                        let tail = Cell {
                            head: Box::new(atom!(n / 2)),
                            tail: self.tail,
                        }.fas()?;
                        if 0 == n % 2 {
                            Cell {
                                head: Box::new(atom!(2)),
                                tail: Box::new(tail),
                            }.fas()
                        } else {
                            Cell {
                                head: Box::new(atom!(3)),
                                tail: Box::new(tail),
                            }.fas()
                        }
                    } else {
                        Err(Error {
                            msg: "/[n b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                },
            }
        } else {
            Err(Error {
                msg: "/[a b] cannot be evaluated when a is a cell".to_string(),
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
        let atom = Atom(777);
        assert_eq!(atom, atom.clone());
    }

    #[test]
    fn clone_cell() {
        // Clone [8 808].
        let cell = Cell {
            head: Box::new(atom!(8)),
            tail: Box::new(atom!(808)),
        };
        assert_eq!(cell, cell.clone());
    }

    #[test]
    fn clone_noun() {
        // Clone 101010.
        let noun = atom!(101010);
        assert_eq!(noun, noun.clone());

        // Clone [300 [400 500]].
        let noun = cell!(
            Box::new(atom!(300)),
            Box::new(cell!(
                Box::new(atom!(400)),
                Box::new(atom!(500)),
            )),
        );
        assert_eq!(noun, noun.clone());
    }

    #[test]
    fn fas_cell() {
        let h1 = Box::new(atom!(1));
        let h2 = Box::new(atom!(2));
        let h3 = Box::new(atom!(3));
        let h5 = Box::new(atom!(5));
        let h6 = Box::new(atom!(6));

        // /[1 [98 89]] -> [98 89]
        let t = Box::new(cell!(
            Box::new(atom!(98)),
            Box::new(atom!(89)),
        ));
        let cell = Cell {
            head: h1.clone(),
            tail: t.clone(),
        };
        match cell.fas() {
            Ok(res) => {
                assert!(*t == res);
            },
            Err(err) => {
                assert!(false, "Unexpected failure: {}.", err.msg);
            },
        }

        // /[2 [292 1001]] -> 292
        let th = Box::new(atom!(292));
        let cell = Cell {
            head: h2.clone(),
            tail: Box::new(cell!(
                th.clone(),
                Box::new(atom!(1001)),
            )),
        };
        match cell.fas() {
            Ok(res) => {
                assert!(*th == res)
            },
            Err(err) => {
                assert!(false, "Unexpected failure: {}.", err.msg);
            },
        }

        // /[2 107] -> error
        let cell = Cell {
            head: h2.clone(),
            tail: Box::new(atom!(107)),
        };
        match cell.fas() {
            Ok(_) => {
                assert!(false, "Unexpected success.");
            },
            Err(_) => {
                assert!(true);
            },
        }

        // /[3 [[80 50] [19 95]]] -> [19 95]
        let tt = Box::new(cell!(
            Box::new(atom!(19)),
            Box::new(atom!(95)),
        ));
        let cell = Cell {
            head: h3.clone(),
            tail: Box::new(cell!(
                Box::new(cell!(
                    Box::new(atom!(80)),
                    Box::new(atom!(50)),
                )),
                tt.clone(),
            )),
        };
        match cell.fas() {
            Ok(res) => {
                assert!(*tt == res)
            },
            Err(err) => {
                assert!(false, "Unexpected failure: {}.", err.msg);
            },
        }

        // /[(2 + 2 + 1) [[15 16] 17]]
        // -> /[3 /[2 [[15 16] 17]]]
        // -> /[3 [15 16]]
        // -> 16
        let tht = Box::new(atom!(16));
        let cell = Cell {
            head: h5.clone(),
            tail: Box::new(cell!(
                Box::new(cell!(
                    Box::new(atom!(15)),
                    tht.clone(),
                )),
                Box::new(atom!(17)),
            )),
        };
        match cell.fas() {
            Ok(res) => {
                assert!(*tht == res)
            },
            Err(err) => {
                assert!(false, "Unexpected failure: {}.", err.msg);
            },
        }

        // /[(3 + 3) [4 [8 12]]] -> /[2 /[3 [4 [8 12]]]] -> 8
        let tth = Box::new(atom!(8));
        let cell = Cell {
            head: h6.clone(),
            tail: Box::new(cell!(
                Box::new(atom!(4)),
                Box::new(cell!(
                    tth.clone(),
                    Box::new(atom!(12)),
                )),
            )),
        };
        match cell.fas() {
            Ok(res) => {
                assert!(*tth == res)
            },
            Err(err) => {
                assert!(false, "Unexpected failure: {}.", err.msg);
            },
        }
    }

    #[test]
    fn lus_atom() {
        // +999 -> 1000
        let atom = Atom(999);
        assert_eq!(1000, atom.lus().0);
    }

    #[test]
    fn partialeq_cell() {
        // [71 109] == [71 109]
        let lh = Cell {
            head: Box::new(atom!(71)),
            tail: Box::new(atom!(109)),
        };
        let rh = Cell {
            head: Box::new(atom!(71)),
            tail: Box::new(atom!(109)),
        };
        assert!(lh == rh);

        // [71 109] != [109 71]
        let lh = Cell {
            head: Box::new(atom!(71)),
            tail: Box::new(atom!(109)),
        };
        let rh = Cell {
            head: Box::new(atom!(109)),
            tail: Box::new(atom!(71)),
        };
        assert!(lh != rh);

        // [11 [12 13]] == [11 [12 13]]
        let lh = Cell {
            head: Box::new(atom!(11)),
            tail: Box::new(cell!(
                Box::new(atom!(12)),
                Box::new(atom!(13)),
            )),
        };
        let rh = Cell {
            head: Box::new(atom!(11)),
            tail: Box::new(cell!(
                Box::new(atom!(12)),
                Box::new(atom!(13)),
            )),
        };
        assert!(lh == rh);

        // [11 [12 13]] != [11 [13 12]]
        let lh = Cell {
            head: Box::new(atom!(11)),
            tail: Box::new(cell!(
                Box::new(atom!(12)),
                Box::new(atom!(13)),
            )),
        };
        let rh = Cell {
            head: Box::new(atom!(11)),
            tail: Box::new(cell!(
                Box::new(atom!(13)),
                Box::new(atom!(12)),
            )),
        };
        assert!(lh != rh);
    }

    #[test]
    fn partialeq_noun() {
        // 500 == 500
        let lh = atom!(500);
        let rh = atom!(500);
        assert!(lh == rh);

        // 499 != 501
        let lh = atom!(499);
        let rh = atom!(501);
        assert!(lh != rh);

        // [0 5] == [0 5]
        let lh = cell!(
            Box::new(atom!(0)),
            Box::new(atom!(5)),
        );
        let rh = cell!(
            Box::new(atom!(0)),
            Box::new(atom!(5)),
        );
        assert!(lh == rh);

        // [0 0] == [0 5]
        let lh = cell!(
            Box::new(atom!(0)),
            Box::new(atom!(0)),
        );
        let rh = cell!(
            Box::new(atom!(0)),
            Box::new(atom!(5)),
        );
        assert!(lh != rh);

        // [[44 22] 88] == [[44 22] 88]
        let lh = cell!(
            Box::new(cell!(
                Box::new(atom!(44)),
                Box::new(atom!(22)),
            )),
            Box::new(atom!(88)),
        );
        let rh = cell!(
            Box::new(cell!(
                Box::new(atom!(44)),
                Box::new(atom!(22)),
            )),
            Box::new(atom!(88)),
        );
        assert!(lh == rh);

        // [[44 22] 88] != [44 [22 88]]
        let lh = cell!(
            Box::new(cell!(
                Box::new(atom!(44)),
                Box::new(atom!(22)),
            )),
            Box::new(atom!(88)),
        );
        let rh = cell!(
            Box::new(atom!(44)),
            Box::new(cell!(
                Box::new(atom!(22)),
                Box::new(atom!(88)),
            )),
        );
        assert!(lh != rh);
    }

    #[test]
    fn tis_cell() {
        // [2 2] -> 0
        let cell = Cell {
            head: Box::new(atom!(2)),
            tail: Box::new(atom!(2)),
        };
        assert_eq!(Loobean::Yes, cell.tis());

        // [7 6] -> 1
        let cell = Cell {
            head: Box::new(atom!(7)),
            tail: Box::new(atom!(6)),
        };
        assert_eq!(Loobean::No, cell.tis());

        // [[2 7] [2 7]] -> 0
        let cell = Cell {
            head: Box::new(cell!(
                Box::new(atom!(2)),
                Box::new(atom!(7)),
            )),
            tail: Box::new(cell!(
                Box::new(atom!(2)),
                Box::new(atom!(7)),
            )),
        };
        assert_eq!(Loobean::Yes, cell.tis());

        // [[2 7] [2 [7 3]]] -> 1
        let cell = Cell {
            head: Box::new(Noun::Cell(Cell {
                head: Box::new(atom!(2)),
                tail: Box::new(atom!(7)),
            })),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(atom!(2)),
                tail: Box::new(Noun::Cell(Cell {
                    head: Box::new(atom!(7)),
                    tail: Box::new(atom!(3)),
                }))
            })),
        };
        assert_eq!(Loobean::No, cell.tis());
    }

    #[test]
    fn wut_atom() {
        // ?137 -> 1
        let atom = Atom(137);
        assert_eq!(Loobean::No, atom.wut());
    }

    #[test]
    fn wut_cell() {
        // ?[128 256] -> 0
        let cell = Cell {
            head: Box::new(atom!(128)),
            tail: Box::new(atom!(256)),
        };
        assert_eq!(Loobean::Yes, cell.wut());

        // ?[[512 1024] [16 32]] -> 0
        let cell = Cell {
            head: Box::new(Noun::Cell(Cell {
                head: Box::new(atom!(512)),
                tail: Box::new(atom!(1024)),
            })),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(atom!(16)),
                tail: Box::new(atom!(32)),
            })),
        };
        assert_eq!(Loobean::Yes, cell.wut());
    }

}
