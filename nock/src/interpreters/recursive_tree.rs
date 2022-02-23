use super::*;

impl Fas for Cell {
    fn fas(self) -> Result<Noun, Error> {
        if let Noun::Atom(h) = *ch!(self) {
            match h {
                Atom::Direct(0) => Err(bad_literal!("/[0 a]", 2)),
                Atom::Direct(1) => Ok(*ct!(self)),
                Atom::Direct(2) => {
                    if let Noun::Cell(t) = *ct!(self) {
                        Ok(*ch!(t))
                    } else {
                        Err(unexpected_atom!("/[2 a]", 3))
                    }
                }
                Atom::Direct(3) => {
                    if let Noun::Cell(t) = *ct!(self) {
                        Ok(*ct!(t))
                    } else {
                        Err(unexpected_atom!("/[3 a]", 3))
                    }
                }
                Atom::Direct(n) => {
                    c!(b!(na!(2 + n % 2)), b!(c!(b!(na!(n / 2)), ct!(self)).fas()?)).fas()
                }
                Atom::Indirect(_) => Err(unexpected_iatom!("/[a b]", 2)),
            }
        } else {
            Err(unexpected_cell!("/[a b]", 2))
        }
    }
}

impl Hax for Cell {
    fn hax(self) -> Result<Noun, Error> {
        if let Noun::Atom(h) = *ch!(self) {
            if let Noun::Cell(t) = *ct!(self) {
                match h {
                    Atom::Direct(0) => Err(bad_literal!("#[0 a b]", 2)),
                    Atom::Direct(1) => Ok(*ch!(t)),
                    Atom::Direct(n) if 0 == n % 2 => c!(
                        b!(na!(n / 2)),
                        b!(nc!(
                            b!(nc!(ch!(t), b!(c!(b!(na!(n + 1)), ct!(t).clone()).fas()?))),
                            ct!(t)
                        ))
                    )
                    .hax(),
                    Atom::Direct(n) => c!(
                        b!(na!(n / 2)),
                        b!(nc!(
                            b!(nc!(b!(c!(b!(na!(n - 1)), ct!(t).clone()).fas()?), ch!(t))),
                            ct!(t)
                        ))
                    )
                    .hax(),
                    Atom::Indirect(_) => Err(unexpected_iatom!("#[a b]", 2)),
                }
            } else {
                Err(unexpected_atom!("#[a b]", 3))
            }
        } else {
            Err(unexpected_cell!("#[a b]", 2))
        }
    }
}

impl Tar for Cell {
    fn tar(self) -> Result<Noun, Error> {
        if let Noun::Cell(t) = *ct!(self) {
            match *ch!(t) {
                Noun::Atom(th) => match th {
                    Atom::Direct(0) => c!(ct!(t), ch!(self)).fas(),
                    Atom::Direct(1) => Ok(*ct!(t)),
                    Atom::Direct(2) => {
                        if let Noun::Cell(tt) = *ct!(t) {
                            c!(
                                b!(c!(ch!(self).clone(), ch!(tt)).tar()?),
                                b!(c!(ch!(self), ct!(tt)).tar()?)
                            )
                            .tar()
                        } else {
                            Err(unexpected_atom!("*[a 2 b]", 7))
                        }
                    }
                    Atom::Direct(3) => match c!(ch!(self), ct!(t)).tar()? {
                        Noun::Atom(a) => Ok(Noun::from_loobean(a.wut())),
                        Noun::Cell(c) => Ok(Noun::from_loobean(c.wut())),
                    },
                    Atom::Direct(4) => {
                        if let Noun::Atom(a) = c!(ch!(self), ct!(t)).tar()? {
                            Ok(Noun::Atom(a.lus()))
                        } else {
                            Err(unexpected_cell!("+a", 1))
                        }
                    }
                    Atom::Direct(5) => {
                        if let Noun::Cell(tt) = *ct!(t) {
                            Ok(Noun::from_loobean(
                                c!(
                                    b!(c!(ch!(self).clone(), ch!(tt)).tar()?),
                                    b!(c!(ch!(self), ct!(tt)).tar()?)
                                )
                                .tis(),
                            ))
                        } else {
                            Err(unexpected_atom!("*[a 5 b]", 7))
                        }
                    }
                    Atom::Direct(6) => {
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
                                Err(unexpected_atom!("*[a 6 b c]", 15))
                            }
                        } else {
                            Err(unexpected_atom!("*[a 6 b]", 7))
                        }
                    }
                    Atom::Direct(7) => {
                        if let Noun::Cell(tt) = *ct!(t) {
                            c!(b!(c!(ch!(self), ch!(tt)).tar()?), ct!(tt)).tar()
                        } else {
                            Err(unexpected_atom!("*[a 7 b]", 7))
                        }
                    }
                    Atom::Direct(8) => {
                        if let Noun::Cell(tt) = *ct!(t) {
                            c!(
                                b!(nc!(b!(c!(ch!(self).clone(), ch!(tt)).tar()?), ch!(self))),
                                ct!(tt)
                            )
                            .tar()
                        } else {
                            Err(unexpected_atom!("*[a 8 b]", 7))
                        }
                    }
                    Atom::Direct(9) => {
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
                            Err(unexpected_atom!("*[a 9 b]", 7))
                        }
                    }
                    Atom::Direct(10) => {
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
                                Err(unexpected_atom!("*[a 10 b c]", 14))
                            }
                        } else {
                            Err(unexpected_atom!("*[a 10 b]", 7))
                        }
                    }
                    Atom::Direct(11) => {
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
                            Err(unexpected_atom!("*[a 11 b]", 7))
                        }
                    }
                    Atom::Direct(n) => Err(bad_literal!(format!("*[a {} b]", n), 6)),
                    Atom::Indirect(_) => Err(unexpected_iatom!("*[a b c]", 6)),
                },
                Noun::Cell(th) => Ok(nc!(
                    b!(c!(ch!(self).clone(), b!(Noun::Cell(th))).tar()?),
                    b!(c!(ch!(self), ct!(t)).tar()?)
                )),
            }
        } else {
            Err(unexpected_atom!("*[a b]", 3))
        }
    }
}
