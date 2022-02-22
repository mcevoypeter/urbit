use super::*;

#[cfg(all(feature = "iterative_tree", feature = "recursive_tree"))]
compile_error!(
    "feature \"iterative_tree\" and \"recursive_tree\" cannot be enabled at the same time"
);

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

#[cfg(feature = "recursive_tree")]
mod recursive_tree {
    use super::*;

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
                                    Cell::new(Box::new(Noun::new_atom(n + 1)), t.t.clone())
                                        .fas()?,
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
                                    Cell::new(Box::new(Noun::new_atom(n - 1)), t.t.clone())
                                        .fas()?,
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
                    msg: "#[a b] cannot be evaluated when a is cell and/or b is an atom"
                        .to_string(),
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
                                                                            Box::new(
                                                                                Noun::new_atom(4),
                                                                            ),
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
}

#[cfg(feature = "iterative_tree")]
mod iterative_tree {
    use super::*;

    impl Fas for Cell {
        fn fas(self) -> Result<Noun, Error> {
            let mut s = self;
            loop {
                match *s.h {
                    Noun::Atom(Atom(0)) => {
                        break Err(Error {
                            msg: "/[0 a] cannot be evaluated".to_string(),
                        })
                    }
                    Noun::Atom(Atom(1)) => break Ok(*s.t),
                    Noun::Atom(Atom(2)) => {
                        break {
                            if let Noun::Cell(t) = *s.t {
                                Ok(*t.h)
                            } else {
                                Err(Error {
                                    msg: "/[2 a] cannot be evaluated when a is an atom".to_string(),
                                })
                            }
                        }
                    }
                    Noun::Atom(Atom(3)) => {
                        break {
                            if let Noun::Cell(t) = *s.t {
                                Ok(*t.t)
                            } else {
                                Err(Error {
                                    msg: "/[3 a] cannot be evaluated when a is an atom".to_string(),
                                })
                            }
                        }
                    }
                    Noun::Atom(Atom(n)) => {
                        s = Cell::new(
                            Box::new(Noun::new_atom(2 + n % 2)),
                            Box::new(Cell::new(Box::new(Noun::new_atom(n / 2)), s.t).fas()?),
                        )
                    }
                    Noun::Cell(_) => {
                        break Err(Error {
                            msg: "/[a b] cannot be evaluated when a is a cell".to_string(),
                        })
                    }
                }
            }
        }
    }

    impl Hax for Cell {
        fn hax(self) -> Result<Noun, Error> {
            let mut s = self;
            loop {
                if let (Noun::Atom(h), Noun::Cell(t)) = (*s.h, *s.t) {
                    match h {
                        Atom(0) => {
                            break Err(Error {
                                msg: "#[0 a b] cannot be evaluated".to_string(),
                            })
                        }
                        Atom(1) => break Ok(*t.h),
                        Atom(n) if 0 == n % 2 => {
                            s = Cell::new(
                                Box::new(Noun::new_atom(n / 2)),
                                Box::new(Noun::new_cell(
                                    Box::new(Noun::new_cell(
                                        t.h,
                                        Box::new(
                                            Cell::new(Box::new(Noun::new_atom(n + 1)), t.t.clone())
                                                .fas()?,
                                        ),
                                    )),
                                    t.t,
                                )),
                            )
                        }
                        Atom(n) => {
                            s = Cell::new(
                                Box::new(Noun::new_atom(n / 2)),
                                Box::new(Noun::new_cell(
                                    Box::new(Noun::new_cell(
                                        Box::new(
                                            Cell::new(Box::new(Noun::new_atom(n - 1)), t.t.clone())
                                                .fas()?,
                                        ),
                                        t.h,
                                    )),
                                    t.t,
                                )),
                            )
                        }
                    }
                } else {
                    break Err(Error {
                        msg: "#[a b] cannot be evaluated when a is cell and/or b is an atom"
                            .to_string(),
                    });
                }
            }
        }
    }

    impl Tar for Cell {
        fn tar(self) -> Result<Noun, Error> {
            let mut s = self;
            loop {
                if let Noun::Cell(t) = *s.t {
                    match *t.h {
                        Noun::Atom(Atom(0)) => break Cell::new(t.t, s.h).fas(),
                        Noun::Atom(Atom(1)) => break Ok(*t.t),
                        Noun::Atom(Atom(2)) => {
                            if let Noun::Cell(tt) = *t.t {
                                s = Cell::new(
                                    Box::new(Cell::new(s.h.clone(), tt.h).tar()?),
                                    Box::new(Cell::new(s.h, tt.t).tar()?),
                                )
                            } else {
                                break Err(Error {
                                    msg: "*[a 2 b] cannot be evaluated when b is an atom"
                                        .to_string(),
                                });
                            }
                        }
                        Noun::Atom(Atom(3)) => {
                            break {
                                match Cell::new(s.h, t.t).tar()? {
                                    Noun::Atom(a) => Ok(Noun::from_loobean(a.wut())),
                                    Noun::Cell(c) => Ok(Noun::from_loobean(c.wut())),
                                }
                            }
                        }
                        Noun::Atom(Atom(4)) => {
                            break {
                                if let Noun::Atom(a) = Cell::new(s.h, t.t).tar()? {
                                    Ok(Noun::Atom(a.lus()))
                                } else {
                                    Err(Error {
                                        msg: "Cannot apply the + operator to a cell".to_string(),
                                    })
                                }
                            }
                        }
                        Noun::Atom(Atom(5)) => {
                            break {
                                if let Noun::Cell(tt) = *t.t {
                                    Ok(Noun::from_loobean(
                                        Cell::new(
                                            Box::new(Cell::new(s.h.clone(), tt.h).tar()?),
                                            Box::new(Cell::new(s.h, tt.t).tar()?),
                                        )
                                        .tis(),
                                    ))
                                } else {
                                    Err(Error {
                                        msg: "*[a 5 b] cannot be evaluated when b is an atom"
                                            .to_string(),
                                    })
                                }
                            }
                        }
                        Noun::Atom(Atom(6)) => {
                            if let Noun::Cell(tt) = *t.t {
                                if let Noun::Cell(ttt) = *tt.t {
                                    s = Cell::new(
                                        s.h.clone(),
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
                                                                                        s.h,
                                                                                        Box::new(Noun::new_cell(
                                                                                                Box::new(Noun::new_atom(4)),
                                                                                                Box::new(Noun::new_cell(
                                                                                                        Box::new(
                                                                                                            Noun::new_atom(4),
                                                                                                            ),
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
                                } else {
                                    break Err(Error {
                                        msg: "*[a 6 b c] cannot be evaluated when c is an atom"
                                            .to_string(),
                                    });
                                }
                            } else {
                                break Err(Error {
                                    msg: "*[a 6 b] cannot be evaluated when b is an atom"
                                        .to_string(),
                                });
                            }
                        }
                        Noun::Atom(Atom(7)) => {
                            if let Noun::Cell(tt) = *t.t {
                                s = Cell::new(Box::new(Cell::new(s.h, tt.h).tar()?), tt.t)
                            } else {
                                break Err(Error {
                                    msg: "*[a 7 b] cannot be evaluated when b is an atom"
                                        .to_string(),
                                });
                            }
                        }
                        Noun::Atom(Atom(8)) => {
                            if let Noun::Cell(tt) = *t.t {
                                s = Cell::new(
                                    Box::new(Noun::new_cell(
                                        Box::new(Cell::new(s.h.clone(), tt.h).tar()?),
                                        s.h,
                                    )),
                                    tt.t,
                                )
                            } else {
                                break Err(Error {
                                    msg: "*[a 8 b] cannot be evaluated when b is an atom"
                                        .to_string(),
                                });
                            }
                        }
                        Noun::Atom(Atom(9)) => {
                            if let Noun::Cell(tt) = *t.t {
                                s = Cell::new(
                                    Box::new(Cell::new(s.h, tt.t).tar()?),
                                    Box::new(Noun::new_cell(
                                        Box::new(Noun::new_atom(2)),
                                        Box::new(Noun::new_cell(
                                            Box::new(Noun::new_cell(
                                                Box::new(Noun::new_atom(0)),
                                                Box::new(Noun::new_atom(1)),
                                            )),
                                            Box::new(Noun::new_cell(
                                                Box::new(Noun::new_atom(0)),
                                                tt.h,
                                            )),
                                        )),
                                    )),
                                )
                            } else {
                                break Err(Error {
                                    msg: "*[a 9 b] cannot be evaluated when b is an atom"
                                        .to_string(),
                                });
                            }
                        }
                        Noun::Atom(Atom(10)) => {
                            break if let Noun::Cell(tt) = *t.t {
                                if let Noun::Cell(tth) = *tt.h {
                                    Cell::new(
                                        tth.h,
                                        Box::new(Noun::new_cell(
                                            Box::new(Cell::new(s.h.clone(), tth.t).tar()?),
                                            Box::new(Cell::new(s.h, tt.t).tar()?),
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
                                    msg: "*[a 10 b] cannot be evaluated when b is an atom"
                                        .to_string(),
                                })
                            }
                        }
                        Noun::Atom(Atom(11)) => {
                            if let Noun::Cell(tt) = *t.t {
                                match *tt.h {
                                    Noun::Atom(_) => break Cell::new(s.h, tt.t).tar(),
                                    Noun::Cell(c) => {
                                        s = Cell::new(
                                            Box::new(Noun::new_cell(
                                                Box::new(Cell::new(s.h.clone(), c.t).tar()?),
                                                Box::new(Cell::new(s.h, tt.t).tar()?),
                                            )),
                                            Box::new(Noun::new_cell(
                                                Box::new(Noun::new_atom(0)),
                                                Box::new(Noun::new_atom(3)),
                                            )),
                                        )
                                    }
                                }
                            } else {
                                break Err(Error {
                                    msg: "*[a 11 b] cannot be evaluated when b is an atom"
                                        .to_string(),
                                });
                            }
                        }
                        Noun::Atom(Atom(_)) => {
                            break Err(Error {
                                msg: "unsupported opcode".to_string(),
                            })
                        }
                        Noun::Cell(th) => {
                            break Ok(Noun::new_cell(
                                Box::new(Cell::new(s.h.clone(), Box::new(Noun::Cell(th))).tar()?),
                                Box::new(Cell::new(s.h, t.t).tar()?),
                            ))
                        }
                    }
                } else {
                    break Err(Error {
                        msg: "*[a b] cannot be evaluated when b is an atom".to_string(),
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrement() {
        // [[1 0] [0 1]] -> [1 0]
        {
            match Cell::new(
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(1)),
                    Box::new(Noun::new_atom(0)),
                )),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(0)),
                    Box::new(Noun::new_atom(1)),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::new_cell(Box::new(Noun::new_atom(1)), Box::new(Noun::new_atom(0)),),
                        res
                    );
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // [42 [1 0] [0 1]] -> [0 42]
        {
            match Cell::new(
                Box::new(Noun::new_atom(42)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(1)),
                        Box::new(Noun::new_atom(0)),
                    )),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(0)),
                        Box::new(Noun::new_atom(1)),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::new_cell(Box::new(Noun::new_atom(0)), Box::new(Noun::new_atom(42)),),
                        res
                    );
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // [4 0 1]
        let increment = Box::new(Noun::new_cell(
            Box::new(Noun::new_atom(4)),
            Box::new(Noun::new_cell(
                Box::new(Noun::new_atom(0)),
                Box::new(Noun::new_atom(1)),
            )),
        ));

        // [8 [1 0] 8 [1 6 [5 [0 7] 4 0 6] [0 6] 9 2 [0 2] [4 0 6] 0 7] 9 2 0 1]
        let decrement = Box::new(Noun::new_cell(
            Box::new(Noun::new_atom(8)),
            Box::new(Noun::new_cell(
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(1)),
                    Box::new(Noun::new_atom(0)),
                )),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(8)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(1)),
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(6)),
                                Box::new(Noun::new_cell(
                                    Box::new(Noun::new_cell(
                                        Box::new(Noun::new_atom(5)),
                                        Box::new(Noun::new_cell(
                                            Box::new(Noun::new_cell(
                                                Box::new(Noun::new_atom(0)),
                                                Box::new(Noun::new_atom(7)),
                                            )),
                                            Box::new(Noun::new_cell(
                                                Box::new(Noun::new_atom(4)),
                                                Box::new(Noun::new_cell(
                                                    Box::new(Noun::new_atom(0)),
                                                    Box::new(Noun::new_atom(6)),
                                                )),
                                            )),
                                        )),
                                    )),
                                    Box::new(Noun::new_cell(
                                        Box::new(Noun::new_cell(
                                            Box::new(Noun::new_atom(0)),
                                            Box::new(Noun::new_atom(6)),
                                        )),
                                        Box::new(Noun::new_cell(
                                            Box::new(Noun::new_atom(9)),
                                            Box::new(Noun::new_cell(
                                                Box::new(Noun::new_atom(2)),
                                                Box::new(Noun::new_cell(
                                                    Box::new(Noun::new_cell(
                                                        Box::new(Noun::new_atom(0)),
                                                        Box::new(Noun::new_atom(2)),
                                                    )),
                                                    Box::new(Noun::new_cell(
                                                        Box::new(Noun::new_cell(
                                                            Box::new(Noun::new_atom(4)),
                                                            Box::new(Noun::new_cell(
                                                                Box::new(Noun::new_atom(0)),
                                                                Box::new(Noun::new_atom(6)),
                                                            )),
                                                        )),
                                                        Box::new(Noun::new_cell(
                                                            Box::new(Noun::new_atom(0)),
                                                            Box::new(Noun::new_atom(7)),
                                                        )),
                                                    )),
                                                )),
                                            )),
                                        )),
                                    )),
                                )),
                            )),
                        )),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(9)),
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(2)),
                                Box::new(Noun::new_cell(
                                    Box::new(Noun::new_atom(0)),
                                    Box::new(Noun::new_atom(1)),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        ));

        // *[42 decrement] -> 41
        {
            match Cell::new(Box::new(Noun::new_atom(42)), decrement.clone()).tar() {
                Ok(res) => {
                    assert_eq!(Noun::new_atom(41), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[107 decrement increment] -> [106 108]
        // TODO: resolve the stack overflow that occurs when this test is run.
        {
            match Cell::new(
                Box::new(Noun::new_atom(107)),
                Box::new(Noun::new_cell(decrement.clone(), increment.clone())),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::new_cell(
                            Box::new(Noun::new_atom(106)),
                            Box::new(Noun::new_atom(108)),
                        ),
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
    fn fas_cell() {
        // /[1 [98 89]] -> [98 89]
        {
            let t = Noun::new_cell(Box::new(Noun::new_atom(98)), Box::new(Noun::new_atom(89)))
                .into_box();
            match Cell::new(Box::new(Noun::new_atom(1)), t.clone()).fas() {
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
            let th = Box::new(Noun::new_atom(292));
            match Cell::new(
                Box::new(Noun::new_atom(2)),
                Box::new(Noun::new_cell(th.clone(), Box::new(Noun::new_atom(1001)))),
            )
            .fas()
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
            match Cell::new(Box::new(Noun::new_atom(2)), Box::new(Noun::new_atom(107))).fas() {
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
            let tt = Noun::new_cell(Box::new(Noun::new_atom(19)), Box::new(Noun::new_atom(95)))
                .into_box();
            match Cell::new(
                Box::new(Noun::new_atom(3)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(80)),
                        Box::new(Noun::new_atom(50)),
                    )),
                    tt.clone(),
                )),
            )
            .fas()
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
            let tht = Box::new(Noun::new_atom(16));
            match Cell::new(
                Box::new(Noun::new_atom(5)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_cell(Box::new(Noun::new_atom(15)), tht.clone())),
                    Box::new(Noun::new_atom(17)),
                )),
            )
            .fas()
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
            let tth = Box::new(Noun::new_atom(8));
            match Cell::new(
                Box::new(Noun::new_atom(6)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(4)),
                    Box::new(Noun::new_cell(tth.clone(), Box::new(Noun::new_atom(12)))),
                )),
            )
            .fas()
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
            match Cell::new(
                Box::new(Noun::new_atom(12)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(531)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(25)),
                        Box::new(Noun::new_atom(99)),
                    )),
                )),
            )
            .fas()
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
            let th = Box::new(Noun::new_atom(22));
            match Cell::new(
                Box::new(Noun::new_atom(1)),
                Box::new(Noun::new_cell(th.clone(), Box::new(Noun::new_atom(80)))),
            )
            .hax()
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
            let th = Box::new(Noun::new_atom(11));
            let ttt = Box::new(Noun::new_atom(33));
            match Cell::new(
                Box::new(Noun::new_atom(2)),
                Box::new(Noun::new_cell(
                    th.clone(),
                    Box::new(Noun::new_cell(Box::new(Noun::new_atom(22)), ttt.clone())),
                )),
            )
            .hax()
            {
                Ok(res) => {
                    assert_eq!(Noun::new_cell(th, ttt), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // #[3 11 [22 33]] -> [22 11]
        {
            let th = Box::new(Noun::new_atom(11));
            let tth = Box::new(Noun::new_atom(22));
            match Cell::new(
                Box::new(Noun::new_atom(3)),
                Box::new(Noun::new_cell(
                    th.clone(),
                    Box::new(Noun::new_cell(tth.clone(), Box::new(Noun::new_atom(33)))),
                )),
            )
            .hax()
            {
                Ok(res) => {
                    assert_eq!(Noun::new_cell(tth, th), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // #[4 11 [[22 33] 44]] -> [[11 33] 44]
        {
            let th = Box::new(Noun::new_atom(11));
            let ttht = Box::new(Noun::new_atom(33));
            let ttt = Box::new(Noun::new_atom(44));
            match Cell::new(
                Box::new(Noun::new_atom(4)),
                Box::new(Noun::new_cell(
                    th.clone(),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(Box::new(Noun::new_atom(22)), ttht.clone())),
                        ttt.clone(),
                    )),
                )),
            )
            .hax()
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::new_cell(Box::new(Noun::new_cell(th, ttht)), ttt,),
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
            let th = Box::new(Noun::new_atom(11));
            let tthh = Box::new(Noun::new_atom(22));
            let ttt = Box::new(Noun::new_atom(44));
            match Cell::new(
                Box::new(Noun::new_atom(5)),
                Box::new(Noun::new_cell(
                    th.clone(),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(tthh.clone(), Box::new(Noun::new_atom(33)))),
                        ttt.clone(),
                    )),
                )),
            )
            .hax()
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::new_cell(Box::new(Noun::new_cell(tthh, th)), ttt,),
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
    fn tar_cell() {
        // *[1 0] -> crash
        {
            match Cell::new(Box::new(Noun::new_atom(1)), Box::new(Noun::new_atom(0))).tar() {
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
            match Cell::new(
                Box::new(Noun::new_atom(4)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(0)),
                        Box::new(Noun::new_atom(0)),
                    )),
                    Box::new(Noun::new_atom(4)),
                )),
            )
            .tar()
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
            let htt = Noun::new_cell(Box::new(Noun::new_atom(14)), Box::new(Noun::new_atom(15)))
                .into_box();
            match Cell::new(
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(4)),
                        Box::new(Noun::new_atom(5)),
                    )),
                    Box::new(Noun::new_cell(Box::new(Noun::new_atom(6)), htt.clone())),
                )),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(0)),
                    Box::new(Noun::new_atom(7)),
                )),
            )
            .tar()
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
            let tt = Noun::new_cell(Box::new(Noun::new_atom(153)), Box::new(Noun::new_atom(218)))
                .into_box();
            match Cell::new(
                Box::new(Noun::new_atom(42)),
                Box::new(Noun::new_cell(Box::new(Noun::new_atom(1)), tt.clone())),
            )
            .tar()
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
            let ttttt =
                Noun::new_cell(Box::new(Noun::new_atom(153)), Box::new(Noun::new_atom(218)))
                    .into_box();
            match Cell::new(
                Box::new(Noun::new_atom(77)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(2)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(1)),
                            Box::new(Noun::new_atom(42)),
                        )),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(1)),
                            Box::new(Noun::new_cell(Box::new(Noun::new_atom(1)), ttttt.clone())),
                        )),
                    )),
                )),
            )
            .tar()
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
            match Cell::new(
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(19)),
                    Box::new(Noun::new_atom(20)),
                )),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(3)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(0)),
                        Box::new(Noun::new_atom(1)),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(Noun::new_atom(0), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[57 [4 0 1]] -> 58
        {
            match Cell::new(
                Box::new(Noun::new_atom(57)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(4)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(0)),
                        Box::new(Noun::new_atom(1)),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(Noun::new_atom(58), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[12 13] 5 [1 17] [0 3]] -> 1
        {
            match Cell::new(
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(12)),
                    Box::new(Noun::new_atom(13)),
                )),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(5)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(1)),
                            Box::new(Noun::new_atom(17)),
                        )),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(0)),
                            Box::new(Noun::new_atom(3)),
                        )),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(Noun::new_atom(1), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [6 [1 0] [4 0 1] [1 233]]] -> 43
        {
            match Cell::new(
                Box::new(Noun::new_atom(42)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(6)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(1)),
                            Box::new(Noun::new_atom(0)),
                        )),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(4)),
                                Box::new(Noun::new_cell(
                                    Box::new(Noun::new_atom(0)),
                                    Box::new(Noun::new_atom(1)),
                                )),
                            )),
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(1)),
                                Box::new(Noun::new_atom(233)),
                            )),
                        )),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(Noun::new_atom(43), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [6 [1 1] [4 0 1] [1 233]]] -> 233
        {
            match Cell::new(
                Box::new(Noun::new_atom(42)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(6)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(1)),
                            Box::new(Noun::new_atom(1)),
                        )),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(4)),
                                Box::new(Noun::new_cell(
                                    Box::new(Noun::new_atom(0)),
                                    Box::new(Noun::new_atom(1)),
                                )),
                            )),
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(1)),
                                Box::new(Noun::new_atom(233)),
                            )),
                        )),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(Noun::new_atom(233), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [7 [4 0 1] [4 0 1]]] -> 44
        {
            match Cell::new(
                Box::new(Noun::new_atom(42)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(7)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(4)),
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(0)),
                                Box::new(Noun::new_atom(1)),
                            )),
                        )),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(4)),
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(0)),
                                Box::new(Noun::new_atom(1)),
                            )),
                        )),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(Noun::new_atom(44), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [8 [4 0 1] [0 1]]] -> [43 42]
        {
            match Cell::new(
                Box::new(Noun::new_atom(42)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(8)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(4)),
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(0)),
                                Box::new(Noun::new_atom(1)),
                            )),
                        )),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(0)),
                            Box::new(Noun::new_atom(1)),
                        )),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::new_cell(Box::new(Noun::new_atom(43)), Box::new(Noun::new_atom(42)),),
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
            match Cell::new(
                Box::new(Noun::new_atom(42)),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(8)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(4)),
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(0)),
                                Box::new(Noun::new_atom(1)),
                            )),
                        )),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(4)),
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(0)),
                                Box::new(Noun::new_atom(3)),
                            )),
                        )),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(Noun::new_atom(43), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[[0 1] 137] 9 2 [0 1]] -> [[0 1] 137]
        {
            match Cell::new(
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(0)),
                        Box::new(Noun::new_atom(1)),
                    )),
                    Box::new(Noun::new_atom(137)),
                )),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(9)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(2)),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(0)),
                            Box::new(Noun::new_atom(1)),
                        )),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::new_cell(
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(0)),
                                Box::new(Noun::new_atom(1)),
                            )),
                            Box::new(Noun::new_atom(137)),
                        ),
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
            match Cell::new(
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(0)),
                        Box::new(Noun::new_atom(2)),
                    )),
                    Box::new(Noun::new_atom(137)),
                )),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(9)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(2)),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(0)),
                            Box::new(Noun::new_atom(1)),
                        )),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::new_cell(Box::new(Noun::new_atom(0)), Box::new(Noun::new_atom(2)),),
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
            match Cell::new(
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(0)),
                        Box::new(Noun::new_atom(3)),
                    )),
                    Box::new(Noun::new_atom(137)),
                )),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(9)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(2)),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(0)),
                            Box::new(Noun::new_atom(1)),
                        )),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(Noun::new_atom(137), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[16 32] 10 [1 0 2] 0 3] -> 16
        {
            match Cell::new(
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(16)),
                    Box::new(Noun::new_atom(32)),
                )),
                Box::new(Noun::new_cell(
                    Box::new(Noun::new_atom(10)),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(1)),
                            Box::new(Noun::new_cell(
                                Box::new(Noun::new_atom(0)),
                                Box::new(Noun::new_atom(2)),
                            )),
                        )),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(0)),
                            Box::new(Noun::new_atom(3)),
                        )),
                    )),
                )),
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(Noun::new_atom(16), res);
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
                Cell::new(Box::new(Noun::new_atom(2)), Box::new(Noun::new_atom(2)),).tis(),
            );
        }

        // [7 6] -> 1
        {
            assert_eq!(
                Loobean::No,
                Cell::new(Box::new(Noun::new_atom(7)), Box::new(Noun::new_atom(6)),).tis(),
            );
        }

        // [[2 7] [2 7]] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                Cell::new(
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(2)),
                        Box::new(Noun::new_atom(7)),
                    )),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(2)),
                        Box::new(Noun::new_atom(7)),
                    )),
                )
                .tis(),
            );
        }

        // [[2 7] [2 [7 3]]] -> 1
        {
            assert_eq!(
                Loobean::No,
                Cell::new(
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(2)),
                        Box::new(Noun::new_atom(7)),
                    )),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(2)),
                        Box::new(Noun::new_cell(
                            Box::new(Noun::new_atom(7)),
                            Box::new(Noun::new_atom(3)),
                        )),
                    )),
                )
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
                Cell::new(Box::new(Noun::new_atom(128)), Box::new(Noun::new_atom(256)),).wut(),
            );
        }

        // ?[[512 1024] [16 32]] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                Cell::new(
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(512)),
                        Box::new(Noun::new_atom(1024)),
                    )),
                    Box::new(Noun::new_cell(
                        Box::new(Noun::new_atom(16)),
                        Box::new(Noun::new_atom(32)),
                    )),
                )
                .wut(),
            );
        }
    }
}
