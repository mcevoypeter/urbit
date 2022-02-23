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
                    s = c!(b!(na!(2 + n % 2)), b!(c!(b!(na!(n / 2)), s.t).fas()?))
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
                        s = c!(
                            b!(na!(n / 2)),
                            b!(nc!(
                                b!(nc!(t.h, b!(c!(b!(na!(n + 1)), t.t.clone()).fas()?))),
                                t.t
                            ))
                        )
                    }
                    Atom(n) => {
                        s = c!(
                            b!(na!(n / 2)),
                            b!(nc!(
                                b!(nc!(b!(c!(b!(na!(n - 1)), t.t.clone()).fas()?), t.h)),
                                t.t
                            ))
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
                    Noun::Atom(Atom(0)) => break c!(t.t, s.h).fas(),
                    Noun::Atom(Atom(1)) => break Ok(*t.t),
                    Noun::Atom(Atom(2)) => {
                        if let Noun::Cell(tt) = *t.t {
                            s = c!(b!(c!(s.h.clone(), tt.h).tar()?), b!(c!(s.h, tt.t).tar()?))
                        } else {
                            break Err(Error {
                                msg: "*[a 2 b] cannot be evaluated when b is an atom".to_string(),
                            });
                        }
                    }
                    Noun::Atom(Atom(3)) => {
                        break {
                            match c!(s.h, t.t).tar()? {
                                Noun::Atom(a) => Ok(Noun::from_loobean(a.wut())),
                                Noun::Cell(c) => Ok(Noun::from_loobean(c.wut())),
                            }
                        }
                    }
                    Noun::Atom(Atom(4)) => {
                        break {
                            if let Noun::Atom(a) = c!(s.h, t.t).tar()? {
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
                                    c!(b!(c!(s.h.clone(), tt.h).tar()?), b!(c!(s.h, tt.t).tar()?))
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
                                s = c!(
                                    s.h.clone(),
                                    b!(c!(
                                        b!(nc!(ttt.h, ttt.t)),
                                        b!(nc!(
                                            b!(na!(0)),
                                            b!(c!(
                                                b!(nc!(b!(na!(2)), b!(na!(3)))),
                                                b!(nc!(
                                                    b!(na!(0)),
                                                    b!(c!(
                                                        s.h,
                                                        b!(nc!(
                                                            b!(na!(4)),
                                                            b!(nc!(b!(na!(4)), tt.h))
                                                        ))
                                                    )
                                                    .tar()?)
                                                ))
                                            )
                                            .tar()?)
                                        ))
                                    )
                                    .tar()?)
                                )
                            } else {
                                break Err(Error {
                                    msg: "*[a 6 b c] cannot be evaluated when c is an atom"
                                        .to_string(),
                                });
                            }
                        } else {
                            break Err(Error {
                                msg: "*[a 6 b] cannot be evaluated when b is an atom".to_string(),
                            });
                        }
                    }
                    Noun::Atom(Atom(7)) => {
                        if let Noun::Cell(tt) = *t.t {
                            s = c!(b!(c!(s.h, tt.h).tar()?), tt.t)
                        } else {
                            break Err(Error {
                                msg: "*[a 7 b] cannot be evaluated when b is an atom".to_string(),
                            });
                        }
                    }
                    Noun::Atom(Atom(8)) => {
                        if let Noun::Cell(tt) = *t.t {
                            s = c!(b!(nc!(b!(c!(s.h.clone(), tt.h).tar()?), s.h)), tt.t)
                        } else {
                            break Err(Error {
                                msg: "*[a 8 b] cannot be evaluated when b is an atom".to_string(),
                            });
                        }
                    }
                    Noun::Atom(Atom(9)) => {
                        if let Noun::Cell(tt) = *t.t {
                            s = c!(
                                b!(c!(s.h, tt.t).tar()?),
                                b!(nc!(
                                    b!(na!(2)),
                                    b!(nc!(
                                        b!(nc!(b!(na!(0)), b!(na!(1)))),
                                        b!(nc!(b!(na!(0)), tt.h))
                                    ))
                                ))
                            )
                        } else {
                            break Err(Error {
                                msg: "*[a 9 b] cannot be evaluated when b is an atom".to_string(),
                            });
                        }
                    }
                    Noun::Atom(Atom(10)) => {
                        break if let Noun::Cell(tt) = *t.t {
                            if let Noun::Cell(tth) = *tt.h {
                                c!(
                                    tth.h,
                                    b!(nc!(
                                        b!(c!(s.h.clone(), tth.t).tar()?),
                                        b!(c!(s.h, tt.t).tar()?)
                                    ))
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
                                Noun::Atom(_) => break c!(s.h, tt.t).tar(),
                                Noun::Cell(c) => {
                                    s = c!(
                                        b!(nc!(
                                            b!(c!(s.h.clone(), c.t).tar()?),
                                            b!(c!(s.h, tt.t).tar()?)
                                        )),
                                        b!(nc!(b!(na!(0)), b!(na!(3))))
                                    )
                                }
                            }
                        } else {
                            break Err(Error {
                                msg: "*[a 11 b] cannot be evaluated when b is an atom".to_string(),
                            });
                        }
                    }
                    Noun::Atom(Atom(_)) => {
                        break Err(Error {
                            msg: "unsupported opcode".to_string(),
                        })
                    }
                    Noun::Cell(th) => {
                        break Ok(nc!(
                            b!(c!(s.h.clone(), b!(Noun::Cell(th))).tar()?),
                            b!(c!(s.h, t.t).tar()?)
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
