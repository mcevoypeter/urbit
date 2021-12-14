#[derive(Debug)]
#[derive(PartialEq)]
enum Loobean {
    Yes,
    No,
}

enum Noun {
    Atom(Atom),
    Cell(Cell),
}

#[derive(PartialEq)]
struct Atom(u64);

struct Cell {
    head: Box<Noun>,
    tail: Box<Noun>,
}

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
    fn fas(&self) {}
}

// #
trait Hax {
    fn hax(&self) {}
}

// *
trait Tar {
    fn tar(&self) {}
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

#[cfg(test)]
mod tests {
    use crate::*;

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
}
