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
                Noun::Atom(Atom(atom.0))
            },
            Noun::Cell(cell) => {
                Noun::Cell(Cell {
                    head: cell.head.clone(),
                    tail: cell.tail.clone(),
                })
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
                            head: boxed_atom!(n / 2),
                            //head: Box::new(Noun::Atom(Atom(n / 2))),
                            tail: self.tail,
                        }.fas()?;
                        if 0 == n % 2 {
                            Cell {
                                head: Box::new(Noun::Atom(Atom(2))),
                                tail: Box::new(tail),
                            }.fas()
                        } else {
                            Cell {
                                head: Box::new(Noun::Atom(Atom(3))),
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
            head: Box::new(Noun::Atom(Atom(8))),
            tail: Box::new(Noun::Atom(Atom(808))),
        };
        assert_eq!(cell, cell.clone());
    }

    #[test]
    fn clone_noun() {
        // Clone 101010.
        let noun = Noun::Atom(Atom(101010));
        assert_eq!(noun, noun.clone());

        // Clone [300 [400 500]].
        let noun = Noun::Cell(Cell {
            head: Box::new(Noun::Atom(Atom(300))),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(400))),
                tail: Box::new(Noun::Atom(Atom(500))),
            })),
        });
        assert_eq!(noun, noun.clone());
    }

    #[test]
    fn fas_cell() {
        let h1 = Box::new(Noun::Atom(Atom(1)));
        let h2 = Box::new(Noun::Atom(Atom(2)));
        let h3 = Box::new(Noun::Atom(Atom(3)));
        let h5 = Box::new(Noun::Atom(Atom(5)));
        let h6 = Box::new(Noun::Atom(Atom(6)));

        // /[1 [98 89]] -> [98 89]
        let t = Box::new(Noun::Cell(Cell {
            head: Box::new(Noun::Atom(Atom(98))),
            tail: Box::new(Noun::Atom(Atom(89))),
        }));
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
        let th = Box::new(Noun::Atom(Atom(292)));
        let cell = Cell {
            head: h2.clone(),
            tail: Box::new(Noun::Cell(Cell {
                head: th.clone(),
                tail: Box::new(Noun::Atom(Atom(1001))),
            })),
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
            tail: Box::new(Noun::Atom(Atom(107))),
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
        let tt = Box::new(Noun::Cell(Cell {
            head: Box::new(Noun::Atom(Atom(19))),
            tail: Box::new(Noun::Atom(Atom(95))),
        }));
        let cell = Cell {
            head: h3.clone(),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Cell(Cell {
                    head: Box::new(Noun::Atom(Atom(80))),
                    tail: Box::new(Noun::Atom(Atom(50))),
                })),
                tail: tt.clone(),
            })),
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
        let tht = Box::new(Noun::Atom(Atom(16)));
        let cell = Cell {
            head: h5.clone(),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Cell(Cell {
                    head: Box::new(Noun::Atom(Atom(15))),
                    tail: tht.clone(),
                })),
                tail: Box::new(Noun::Atom(Atom(17))),
            })),
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
        let tth = Box::new(Noun::Atom(Atom(8)));
        let cell = Cell {
            head: h6.clone(),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(4))),
                tail: Box::new(Noun::Cell(Cell {
                    head: tth.clone(),
                    tail: Box::new(Noun::Atom(Atom(12))),
                })),
            })),
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
            head: Box::new(Noun::Atom(Atom(71))),
            tail: Box::new(Noun::Atom(Atom(109))),
        };
        let rh = Cell {
            head: Box::new(Noun::Atom(Atom(71))),
            tail: Box::new(Noun::Atom(Atom(109))),
        };
        assert!(lh == rh);

        // [71 109] != [109 71]
        let lh = Cell {
            head: Box::new(Noun::Atom(Atom(71))),
            tail: Box::new(Noun::Atom(Atom(109))),
        };
        let rh = Cell {
            head: Box::new(Noun::Atom(Atom(109))),
            tail: Box::new(Noun::Atom(Atom(71))),
        };
        assert!(lh != rh);

        // [11 [12 13]] == [11 [12 13]]
        let lh = Cell {
            head: Box::new(Noun::Atom(Atom(11))),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(12))),
                tail: Box::new(Noun::Atom(Atom(13))),
            })),
        };
        let rh = Cell {
            head: Box::new(Noun::Atom(Atom(11))),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(12))),
                tail: Box::new(Noun::Atom(Atom(13))),
            })),
        };
        assert!(lh == rh);

        // [11 [12 13]] != [11 [13 12]]
        let lh = Cell {
            head: Box::new(Noun::Atom(Atom(11))),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(12))),
                tail: Box::new(Noun::Atom(Atom(13))),
            })),
        };
        let rh = Cell {
            head: Box::new(Noun::Atom(Atom(11))),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(13))),
                tail: Box::new(Noun::Atom(Atom(12))),
            })),
        };
        assert!(lh != rh);
    }

    #[test]
    fn partialeq_noun() {
        // 500 == 500
        let lh = Noun::Atom(Atom(500));
        let rh = Noun::Atom(Atom(500));
        assert!(lh == rh);

        // 499 != 501
        let lh = Noun::Atom(Atom(499));
        let rh = Noun::Atom(Atom(501));
        assert!(lh != rh);

        // [0 5] == [0 5]
        let lh = Noun::Cell(Cell {
            head: Box::new(Noun::Atom(Atom(0))),
            tail: Box::new(Noun::Atom(Atom(5))),
        });
        let rh = Noun::Cell(Cell {
            head: Box::new(Noun::Atom(Atom(0))),
            tail: Box::new(Noun::Atom(Atom(5))),
        });
        assert!(lh == rh);

        // [0 0] == [0 5]
        let lh = Noun::Cell(Cell {
            head: Box::new(Noun::Atom(Atom(0))),
            tail: Box::new(Noun::Atom(Atom(0))),
        });
        let rh = Noun::Cell(Cell {
            head: Box::new(Noun::Atom(Atom(0))),
            tail: Box::new(Noun::Atom(Atom(5))),
        });
        assert!(lh != rh);

        // [[44 22] 88] == [[44 22] 88]
        let lh = Noun::Cell(Cell {
            head: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(44))),
                tail: Box::new(Noun::Atom(Atom(22))),
            })),
            tail: Box::new(Noun::Atom(Atom(88))),
        });
        let rh = Noun::Cell(Cell {
            head: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(44))),
                tail: Box::new(Noun::Atom(Atom(22))),
            })),
            tail: Box::new(Noun::Atom(Atom(88))),
        });
        assert!(lh == rh);

        // [[44 22] 88] != [44 [22 88]]
        let lh = Noun::Cell(Cell {
            head: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(44))),
                tail: Box::new(Noun::Atom(Atom(22))),
            })),
            tail: Box::new(Noun::Atom(Atom(88))),
        });
        let rh = Noun::Cell(Cell {
            head: Box::new(Noun::Atom(Atom(44))),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(22))),
                tail: Box::new(Noun::Atom(Atom(88))),
            })),
        });
        assert!(lh != rh);
    }

    #[test]
    fn tis_cell() {
        // [2 2] -> 0
        let cell = Cell {
            head: Box::new(Noun::Atom(Atom(2))),
            tail: Box::new(Noun::Atom(Atom(2))),
        };
        assert_eq!(Loobean::Yes, cell.tis());

        // [7 6] -> 1
        let cell = Cell {
            head: Box::new(Noun::Atom(Atom(7))),
            tail: Box::new(Noun::Atom(Atom(6))),
        };
        assert_eq!(Loobean::No, cell.tis());

        // [[2 7] [2 7]] -> 0
        let cell = Cell {
            head: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(2))),
                tail: Box::new(Noun::Atom(Atom(7))),
            })),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(2))),
                tail: Box::new(Noun::Atom(Atom(7))),
            })),
        };
        assert_eq!(Loobean::Yes, cell.tis());

        // [[2 7] [2 [7 3]]] -> 1
        let cell = Cell {
            head: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(2))),
                tail: Box::new(Noun::Atom(Atom(7))),
            })),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(2))),
                tail: Box::new(Noun::Cell(Cell {
                    head: Box::new(Noun::Atom(Atom(7))),
                    tail: Box::new(Noun::Atom(Atom(3))),
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
            head: Box::new(Noun::Atom(Atom(128))),
            tail: Box::new(Noun::Atom(Atom(256))),
        };
        assert_eq!(Loobean::Yes, cell.wut());

        // ?[[512 1024] [16 32]] -> 0
        let cell = Cell {
            head: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(512))),
                tail: Box::new(Noun::Atom(Atom(1024))),
            })),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(16))),
                tail: Box::new(Noun::Atom(Atom(32))),
            })),
        };
        assert_eq!(Loobean::Yes, cell.wut());
    }

}
