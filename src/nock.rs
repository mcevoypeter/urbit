use std::{error, fmt};

mod interpreters;

/*==============================================================================
 * Nock struct and enum definitions
 */

#[derive(Debug)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Atom(u64);

#[derive(Debug)]
pub struct Cell {
    h: Box<Noun>,
    t: Box<Noun>,
}

#[derive(Debug, PartialEq)]
pub enum Loobean {
    Yes,
    No,
}

#[derive(Debug)]
struct Error {
    msg: String,
}

/*==============================================================================
 * General implementations for Noun enum
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
            Loobean::Yes => Noun::Atom(Atom(0)),
            Loobean::No => Noun::Atom(Atom(1)),
        }
    }

    fn into_box(self) -> Box<Self> {
        Box::new(self)
    }
}

/*==============================================================================
 * General implementations for Atom struct
 */

impl Atom {}

/*==============================================================================
 * General implementations for Cell struct
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

impl Cell {}

/*==============================================================================
 * General implementations for Loobean enum
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
 * General implementations for Error struct
 */

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for Error {}

/*==============================================================================
 * Tests
 */

#[cfg(test)]
mod tests {
    use crate::nock::*;

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
                h: Noun::Atom(Atom(8)).into_box(),
                t: Noun::Atom(Atom(808)).into_box(),
            };
            assert_eq!(c, c.clone());
        }
    }

    #[test]
    fn clone_noun() {
        // Clone 101010.
        {
            let noun = Noun::Atom(Atom(101010));
            assert_eq!(noun, noun.clone());
        }

        // Clone [300 [400 500]].
        {
            let noun = Noun::Cell(Cell {
                h: Noun::Atom(Atom(300)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(400)).into_box(),
                    t: Noun::Atom(Atom(500)).into_box(),
                })
                .into_box(),
            });
            assert_eq!(noun, noun.clone());
        }
    }

    #[test]
    fn partialeq_cell() {
        // [71 109] == [71 109]
        {
            assert_eq!(
                Cell {
                    h: Noun::Atom(Atom(71)).into_box(),
                    t: Noun::Atom(Atom(109)).into_box(),
                },
                Cell {
                    h: Noun::Atom(Atom(71)).into_box(),
                    t: Noun::Atom(Atom(109)).into_box(),
                },
            );
        }

        // [71 109] != [109 71]
        {
            assert_ne!(
                Cell {
                    h: Noun::Atom(Atom(71)).into_box(),
                    t: Noun::Atom(Atom(109)).into_box(),
                },
                Cell {
                    h: Noun::Atom(Atom(109)).into_box(),
                    t: Noun::Atom(Atom(71)).into_box(),
                },
            );
        }

        // [11 [12 13]] == [11 [12 13]]
        {
            assert_eq!(
                Cell {
                    h: Noun::Atom(Atom(11)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(12)).into_box(),
                        t: Noun::Atom(Atom(13)).into_box(),
                    })
                    .into_box(),
                },
                Cell {
                    h: Noun::Atom(Atom(11)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(12)).into_box(),
                        t: Noun::Atom(Atom(13)).into_box(),
                    })
                    .into_box(),
                },
            );
        }

        // [11 [12 13]] != [11 [13 12]]
        {
            assert_ne!(
                Cell {
                    h: Noun::Atom(Atom(11)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(12)).into_box(),
                        t: Noun::Atom(Atom(13)).into_box(),
                    })
                    .into_box(),
                },
                Cell {
                    h: Noun::Atom(Atom(11)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(13)).into_box(),
                        t: Noun::Atom(Atom(12)).into_box(),
                    })
                    .into_box(),
                },
            );
        }
    }

    #[test]
    fn partialeq_noun() {
        // 500 == 500
        {
            assert_eq!(Atom(500), Atom(500));
        }

        // 499 != 501
        {
            assert_ne!(Atom(499), Atom(501));
        }

        // [0 5] == [0 5]
        {
            assert_eq!(
                Noun::Cell(Cell {
                    h: Noun::Atom(Atom(0)).into_box(),
                    t: Noun::Atom(Atom(5)).into_box(),
                }),
                Noun::Cell(Cell {
                    h: Noun::Atom(Atom(0)).into_box(),
                    t: Noun::Atom(Atom(5)).into_box(),
                }),
            );
        }

        // [0 0] == [0 5]
        {
            assert_ne!(
                Noun::Cell(Cell {
                    h: Noun::Atom(Atom(0)).into_box(),
                    t: Noun::Atom(Atom(0)).into_box(),
                }),
                Noun::Cell(Cell {
                    h: Noun::Atom(Atom(0)).into_box(),
                    t: Noun::Atom(Atom(5)).into_box(),
                }),
            );
        }

        // [[44 22] 88] == [[44 22] 88]
        {
            assert_eq!(
                Noun::Cell(Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(44)).into_box(),
                        t: Noun::Atom(Atom(22)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Atom(Atom(88)).into_box(),
                }),
                Noun::Cell(Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(44)).into_box(),
                        t: Noun::Atom(Atom(22)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Atom(Atom(88)).into_box(),
                }),
            );
        }

        // [[44 22] 88] != [44 [22 88]]
        {
            assert_ne!(
                Noun::Cell(Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(44)).into_box(),
                        t: Noun::Atom(Atom(22)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Atom(Atom(88)).into_box(),
                }),
                Noun::Cell(Cell {
                    h: Noun::Atom(Atom(44)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(22)).into_box(),
                        t: Noun::Atom(Atom(88)).into_box(),
                    })
                    .into_box(),
                }),
            );
        }
    }
}
