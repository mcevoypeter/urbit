use super::*;

impl Fas for Cell {
    fn fas(self) -> Result<Noun, Error> {
        if let Noun::Atom(h) = *ch!(self) {
            match av!(h) {
                0 => Err(Error {
                    msg: "/[0 a] cannot be evaluated".to_string(),
                }),
                1 => Ok(*ct!(self)),
                2 => {
                    if let Noun::Cell(t) = *ct!(self) {
                        Ok(*ch!(t))
                    } else {
                        Err(Error {
                            msg: "/[2 a] cannot be evaluated when a is an atom".to_string(),
                        })
                    }
                }
                3 => {
                    if let Noun::Cell(t) = *ct!(self) {
                        Ok(*ct!(t))
                    } else {
                        Err(Error {
                            msg: "/[3 a] cannot be evaluated when a is an atom".to_string(),
                        })
                    }
                }
                n => c!(b!(na!(2 + n % 2)), b!(c!(b!(na!(n / 2)), ct!(self)).fas()?)).fas(),
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
        if let (Noun::Atom(h), Noun::Cell(t)) = (*ch!(self), *ct!(self)) {
            match h {
                Atom(0) => Err(Error {
                    msg: "#[0 a b] cannot be evaluated".to_string(),
                }),
                Atom(1) => Ok(*ch!(t)),
                Atom(n) if 0 == n % 2 => c!(
                    b!(na!(n / 2)),
                    b!(nc!(
                        b!(nc!(ch!(t), b!(c!(b!(na!(n + 1)), ct!(t).clone()).fas()?))),
                        ct!(t)
                    ))
                )
                .hax(),
                Atom(n) => c!(
                    b!(na!(n / 2)),
                    b!(nc!(
                        b!(nc!(b!(c!(b!(na!(n - 1)), ct!(t).clone()).fas()?), ch!(t))),
                        ct!(t)
                    ))
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
        if let Noun::Cell(t) = *ct!(self) {
            match *ch!(t) {
                Noun::Atom(Atom(0)) => c!(ct!(t), ch!(self)).fas(),
                Noun::Atom(Atom(1)) => Ok(*ct!(t)),
                Noun::Atom(Atom(2)) => {
                    if let Noun::Cell(tt) = *ct!(t) {
                        c!(
                            b!(c!(ch!(self).clone(), ch!(tt)).tar()?),
                            b!(c!(ch!(self), ct!(tt)).tar()?)
                        )
                        .tar()
                    } else {
                        Err(Error {
                            msg: "*[a 2 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(3)) => match c!(ch!(self), ct!(t)).tar()? {
                    Noun::Atom(a) => Ok(Noun::from_loobean(a.wut())),
                    Noun::Cell(c) => Ok(Noun::from_loobean(c.wut())),
                },
                Noun::Atom(Atom(4)) => {
                    if let Noun::Atom(a) = c!(ch!(self), ct!(t)).tar()? {
                        Ok(Noun::Atom(a.lus()))
                    } else {
                        Err(Error {
                            msg: "Cannot apply the + operator to a cell".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(5)) => {
                    if let Noun::Cell(tt) = *ct!(t) {
                        Ok(Noun::from_loobean(
                            c!(
                                b!(c!(ch!(self).clone(), ch!(tt)).tar()?),
                                b!(c!(ch!(self), ct!(tt)).tar()?)
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
                    if let Noun::Cell(tt) = *ct!(t) {
                        if let Noun::Cell(ttt) = *ct!(tt) {
                            c!(
                                ch!(self).clone(),
                                b!(c!(
                                    b!(nc!(ch!(ttt), ct!(ttt))),
                                    b!(nc!(
                                        b!(na!(0)),
                                        b!(c!(
                                            b!(nc!(b!(na!(2)), b!(na!(3)))),
                                            b!(nc!(
                                                b!(na!(0)),
                                                b!(c!(
                                                    ch!(self),
                                                    b!(nc!(
                                                        b!(na!(4)),
                                                        b!(nc!(b!(na!(4)), ch!(tt)))
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
                    if let Noun::Cell(tt) = *ct!(t) {
                        c!(b!(c!(ch!(self), ch!(tt)).tar()?), ct!(tt)).tar()
                    } else {
                        Err(Error {
                            msg: "*[a 7 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(8)) => {
                    if let Noun::Cell(tt) = *ct!(t) {
                        c!(
                            b!(nc!(b!(c!(ch!(self).clone(), ch!(tt)).tar()?), ch!(self))),
                            ct!(tt)
                        )
                        .tar()
                    } else {
                        Err(Error {
                            msg: "*[a 8 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(9)) => {
                    if let Noun::Cell(tt) = *ct!(t) {
                        c!(
                            b!(c!(ch!(self), ct!(tt)).tar()?),
                            b!(nc!(
                                b!(na!(2)),
                                b!(nc!(
                                    b!(nc!(b!(na!(0)), b!(na!(1)))),
                                    b!(nc!(b!(na!(0)), ch!(tt)))
                                ))
                            ))
                        )
                        .tar()
                    } else {
                        Err(Error {
                            msg: "*[a 9 b] cannot be evaluated when b is an atom".to_string(),
                        })
                    }
                }
                Noun::Atom(Atom(10)) => {
                    if let Noun::Cell(tt) = *ct!(t) {
                        if let Noun::Cell(tth) = *ch!(tt) {
                            c!(
                                ch!(tth),
                                b!(nc!(
                                    b!(c!(ch!(self).clone(), ct!(tth)).tar()?),
                                    b!(c!(ch!(self), ct!(tt)).tar()?)
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
                    if let Noun::Cell(tt) = *ct!(t) {
                        match *ch!(tt) {
                            Noun::Atom(_) => c!(ch!(self), ct!(tt)).tar(),
                            Noun::Cell(c) => c!(
                                b!(nc!(
                                    b!(c!(ch!(self).clone(), ct!(c)).tar()?),
                                    b!(c!(ch!(self), ct!(tt)).tar()?)
                                )),
                                b!(nc!(b!(na!(0)), b!(na!(3))))
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
                Noun::Cell(th) => Ok(nc!(
                    b!(c!(ch!(self).clone(), b!(Noun::Cell(th))).tar()?),
                    b!(c!(ch!(self), ct!(t)).tar()?)
                )),
            }
        } else {
            Err(Error {
                msg: "*[a b] cannot be evaluated when b is an atom".to_string(),
            })
        }
    }
}
