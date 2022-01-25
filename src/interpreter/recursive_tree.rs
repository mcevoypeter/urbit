#![allow(unused_parens)]

use crate::interpreter::*;

/*==============================================================================
 * Nock operator trait implementations for Cell struct
 */

impl Fas for Cell {
    fn fas(self) -> Result<Noun, Error> {
        if let Noun::Atom(h) = *self.h {
            match h {
                Atom(0) => Err(Error {
                    msg: "/[0 a] cannot be evaluated".to_string(),
                }),
                Atom(1) => Ok(*self.t),
                Atom(2) => {
                    if let Noun::Cell(t) = *self.t {
                        Ok(*t.h)
                    } else {
                        Err(Error {
                            msg: "/[2 a] cannot be evaluated when a is an atom".to_string(),
                        })
                    }
                }
                Atom(3) => {
                    if let Noun::Cell(t) = *self.t {
                        Ok(*t.t)
                    } else {
                        Err(Error {
                            msg: "/[3 a] cannot be evaluated when a is an atom".to_string(),
                        })
                    }
                }
                Atom(n) => Cell::new(
                    Box::new(Noun::new_atom(2 + n % 2)),
                    Box::new(Cell::new(Box::new(Noun::new_atom(n / 2)), self.t).fas()?),
                )
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
            match h {
                Atom(0) => Err(Error {
                    msg: "#[0 a b] cannot be evaluated".to_string(),
                }),
                Atom(1) => Ok(*t.h),
                Atom(n) if 0 == n % 2 => Cell::new(
                    Noun::Atom(Atom(n / 2)).into_box(),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(
                            t.h,
                            Box::new(
                                Cell::new(Box::new(Noun::new_atom(n + 1)), t.t.clone()).fas()?,
                            ),
                        )),
                        t.t,
                    )),
                )
                .hax(),
                Atom(n) => Cell::new(
                    Box::new(Noun::new_atom(n / 2)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(
                            Box::new(
                                Cell::new(Box::new(Noun::new_atom(n - 1)), t.t.clone()).fas()?,
                            ),
                            t.h,
                        )),
                        t.t,
                    )),
                )
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
            match *t.h {
                Noun::Atom(Atom(0)) => Cell::new(t.t, self.h).fas(),
                Noun::Atom(Atom(1)) => Ok(*t.t),
                Noun::Atom(Atom(2)) => {
                    if let Noun::Cell(tt) = *t.t {
                        Cell::new(
                            Box::new(Cell::new(self.h.clone(), tt.h).tar()?),
                            Box::new(Cell::new(self.h, tt.t).tar()?),
                        )
                        .tar()
                    } else {
                        Err(Error {
                            msg: "*[a 2 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(3)) => match Cell::new(self.h, t.t).tar()? {
                    Noun::Atom(a) => Ok(Noun::from_loobean(a.wut())),
                    Noun::Cell(c) => Ok(Noun::from_loobean(c.wut())),
                },
                Noun::Atom(Atom(4)) => {
                    if let Noun::Atom(a) = Cell::new(self.h, t.t).tar()? {
                        Ok(Noun::Atom(a.lus()))
                    } else {
                        Err(Error {
                            msg: "Cannot apply the + operator to a cell".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(5)) => {
                    if let Noun::Cell(tt) = *t.t {
                        Ok(Noun::from_loobean(
                            Cell::new(
                                Box::new(Cell::new(self.h.clone(), tt.h).tar()?),
                                Box::new(Cell::new(self.h, tt.t).tar()?),
                            )
                            .tis(),
                        ))
                    } else {
                        Err(Error {
                            msg: "*[a 5 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(6)) => {
                    if let Noun::Cell(tt) = *t.t {
                        if let Noun::Cell(ttt) = *tt.t {
                            Cell::new(
                                self.h.clone(),
                                Box::new(
                                    Cell::new(
                                        Box::new(Noun::new_cell(ttt.h, ttt.t)),
                                        Box::new(Noun::new_cell(
                                            Box::new(Noun::new_atom(0)),
                                            Box::new(
                                                Cell::new(
                                                    Box::new(Noun::new_cell(
                                                        Box::new(Noun::new_atom(2)),
                                                        Box::new(Noun::new_atom(3)),
                                                    )),
                                                    Box::new(Noun::new_cell(
                                                        Box::new(Noun::new_atom(0)),
                                                        Box::new(
                                                            Cell::new(
                                                                self.h,
                                                                Box::new(Noun::new_cell(
                                                                    Box::new(Noun::new_atom(4)),
                                                                    Box::new(Noun::new_cell(
                                                                        Box::new(Noun::new_atom(4)),
                                                                        tt.h,
                                                                    )),
                                                                )),
                                                            )
                                                            .tar()?,
                                                        ),
                                                    )),
                                                )
                                                .tar()?,
                                            ),
                                        )),
                                    )
                                    .tar()?,
                                ),
                            )
                            .tar()
                        } else {
                            Err(Error {
                                msg: "*[a 6 b c] cannot be evaluated when c is an atom".to_string(),
                            })
                        }
                    } else {
                        Err(Error {
                            msg: "*[a 6 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(7)) => {
                    if let Noun::Cell(tt) = *t.t {
                        Cell::new(Box::new(Cell::new(self.h, tt.h).tar()?), tt.t).tar()
                    } else {
                        Err(Error {
                            msg: "*[a 7 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(8)) => {
                    if let Noun::Cell(tt) = *t.t {
                        Cell::new(
                            Box::new(Noun::new_cell(
                                Box::new(Cell::new(self.h.clone(), tt.h).tar()?),
                                self.h,
                            )),
                            tt.t,
                        )
                        .tar()
                    } else {
                        Err(Error {
                            msg: "*[a 8 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(9)) => {
                    if let Noun::Cell(tt) = *t.t {
                        Cell::new(
                            Box::new(Cell::new(self.h, tt.t).tar()?),
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(2)),
                                Box::new(Noun::new_cell(
                                    Box::new(Noun::new_cell(
                                        Box::new(Noun::new_atom(0)),
                                        Box::new(Noun::new_atom(1)),
                                    )),
                                    Box::new(Noun::new_cell(Box::new(Noun::new_atom(0)), tt.h)),
                                )),
                            )),
                        )
                        .tar()
                    } else {
                        Err(Error {
                            msg: "*[a 9 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(10)) => {
                    if let Noun::Cell(tt) = *t.t {
                        if let Noun::Cell(tth) = *tt.h {
                            Cell::new(
                                tth.h,
                                Box::new(Noun::new_cell(
                                    Box::new(Cell::new(self.h.clone(), tth.t).tar()?),
                                    Box::new(Cell::new(self.h, tt.t).tar()?),
                                )),
                            )
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
                Noun::Atom(Atom(11)) => {
                    if let Noun::Cell(tt) = *t.t {
                        match *tt.h {
                            Noun::Atom(_) => Cell::new(self.h, tt.t).tar(),
                            Noun::Cell(c) => Cell::new(
                                Box::new(Noun::new_cell(
                                    Box::new(Cell::new(self.h.clone(), c.t).tar()?),
                                    Box::new(Cell::new(self.h, tt.t).tar()?),
                                )),
                                Box::new(Noun::new_cell(
                                    Box::new(Noun::new_atom(0)),
                                    Box::new(Noun::new_atom(3)),
                                )),
                            )
                            .tar(),
                        }
                    } else {
                        Err(Error {
                            msg: "*[a 11 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(_)) => Err(Error {
                    msg: "unsupported opcode".to_string(),
                }),
                Noun::Cell(th) => Ok(Noun::new_cell(
                    Box::new(Cell::new(self.h.clone(), Box::new(Noun::Cell(th))).tar()?),
                    Box::new(Cell::new(self.h, t.t).tar()?),
                )),
            }
        } else {
            Err(Error {
                msg: "*[a b] cannot be evaluated when b is an atom".to_string(),
            })
        }
    }
}
