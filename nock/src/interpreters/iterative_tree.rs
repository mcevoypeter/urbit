use super::*;

impl Fas for Cell {
    fn fas(self) -> Result<Noun, Error> {
        let mut s = self;
        loop {
            if let Noun::Atom(h) = *ch!(s) {
                match h {
                    Atom::Direct(0) => break Err(bad_literal!("/[0 a]", 2)),
                    Atom::Direct(1) => break Ok(*ct!(s)),
                    Atom::Direct(2) => {
                        break {
                            if let Noun::Cell(t) = *ct!(s) {
                                Ok(*ch!(t))
                            } else {
                                Err(unexpected_atom!("/[2 a]", 2))
                            }
                        }
                    }
                    Atom::Direct(3) => {
                        break {
                            if let Noun::Cell(t) = *ct!(s) {
                                Ok(*ct!(t))
                            } else {
                                Err(unexpected_atom!("/[3 a]", 2))
                            }
                        }
                    }
                    Atom::Direct(n) => {
                        s = c!(b!(na!(2 + n % 2)), b!(c!(b!(na!(n / 2)), ct!(s)).fas()?))
                    }
                    Atom::Indirect(_) => break Err(unexpected_iatom!("/[a b]", 2)),
                }
            } else {
                break Err(unexpected_cell!("/[a b]", 2));
            }
        }
    }
}

impl Hax for Cell {
    fn hax(self) -> Result<Noun, Error> {
        let mut s = self;
        loop {
            if let Noun::Atom(h) = *ch!(s) {
                if let Noun::Cell(t) = *ct!(s) {
                    match h {
                        Atom::Direct(0) => break Err(bad_literal!("#[0 a b]", 2)),
                        Atom::Direct(1) => break Ok(*ch!(t)),
                        Atom::Direct(n) if 0 == n % 2 => {
                            s = c!(
                                b!(na!(n / 2)),
                                b!(nc!(
                                    b!(nc!(ch!(t), b!(c!(b!(na!(n + 1)), ct!(t).clone()).fas()?))),
                                    ct!(t)
                                ))
                            )
                        }
                        Atom::Direct(n) => {
                            s = c!(
                                b!(na!(n / 2)),
                                b!(nc!(
                                    b!(nc!(b!(c!(b!(na!(n - 1)), ct!(t).clone()).fas()?), ch!(t))),
                                    ct!(t)
                                ))
                            )
                        }
                        Atom::Indirect(_) => break Err(unexpected_iatom!("#[a b]", 2)),
                    }
                } else {
                    break Err(unexpected_atom!("#[a b]", 3));
                }
            } else {
                break Err(unexpected_cell!("#[a b]", 2));
            }
        }
    }
}

impl Tar for Cell {
    fn tar(self) -> Result<Noun, Error> {
        let mut s = self;
        loop {
            if let Noun::Cell(t) = *ct!(s) {
                match *ch!(t) {
                    Noun::Atom(th) => match th {
                        Atom::Direct(0) => break c!(ct!(t), ch!(s)).fas(),
                        Atom::Direct(1) => break Ok(*ct!(t)),
                        Atom::Direct(2) => {
                            if let Noun::Cell(tt) = *ct!(t) {
                                s = c!(
                                    b!(c!(ch!(s).clone(), ch!(tt)).tar()?),
                                    b!(c!(ch!(s), ct!(tt)).tar()?)
                                )
                            } else {
                                break Err(unexpected_atom!("*[a 2 b]", 7));
                            }
                        }
                        Atom::Direct(3) => {
                            break {
                                match c!(ch!(s), ct!(t)).tar()? {
                                    Noun::Atom(a) => Ok(Noun::from_loobean(a.wut())),
                                    Noun::Cell(c) => Ok(Noun::from_loobean(c.wut())),
                                }
                            }
                        }
                        Atom::Direct(4) => {
                            break {
                                if let Noun::Atom(a) = c!(ch!(s), ct!(t)).tar()? {
                                    Ok(Noun::Atom(a.lus()))
                                } else {
                                    Err(unexpected_cell!("+a", 1))
                                }
                            }
                        }
                        Atom::Direct(5) => {
                            break {
                                if let Noun::Cell(tt) = *ct!(t) {
                                    Ok(Noun::from_loobean(
                                        c!(
                                            b!(c!(ch!(s).clone(), ch!(tt)).tar()?),
                                            b!(c!(ch!(s), ct!(tt)).tar()?)
                                        )
                                        .tis(),
                                    ))
                                } else {
                                    Err(unexpected_atom!("*[a 5 b]", 7))
                                }
                            }
                        }
                        Atom::Direct(6) => {
                            if let Noun::Cell(tt) = *ct!(t) {
                                if let Noun::Cell(ttt) = *ct!(tt) {
                                    s = c!(
                                        ch!(s).clone(),
                                        b!(c!(
                                            b!(nc!(ch!(ttt), ct!(ttt))),
                                            b!(nc!(
                                                b!(na!(0)),
                                                b!(c!(
                                                    b!(nc!(b!(na!(2)), b!(na!(3)))),
                                                    b!(nc!(
                                                        b!(na!(0)),
                                                        b!(c!(
                                                            ch!(s),
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
                                } else {
                                    break Err(unexpected_atom!("*[a 6 b c]", 15));
                                }
                            } else {
                                break Err(unexpected_atom!("*[a 6 b]", 7));
                            }
                        }
                        Atom::Direct(7) => {
                            if let Noun::Cell(tt) = *ct!(t) {
                                s = c!(b!(c!(ch!(s), ch!(tt)).tar()?), ct!(tt))
                            } else {
                                break Err(unexpected_atom!("*[a 7 b]", 7));
                            }
                        }
                        Atom::Direct(8) => {
                            if let Noun::Cell(tt) = *ct!(t) {
                                s = c!(
                                    b!(nc!(b!(c!(ch!(s).clone(), ch!(tt)).tar()?), ch!(s))),
                                    ct!(tt)
                                )
                            } else {
                                break Err(unexpected_atom!("*[a 8 b]", 7));
                            }
                        }
                        Atom::Direct(9) => {
                            if let Noun::Cell(tt) = *ct!(t) {
                                s = c!(
                                    b!(c!(ch!(s), ct!(tt)).tar()?),
                                    b!(nc!(
                                        b!(na!(2)),
                                        b!(nc!(
                                            b!(nc!(b!(na!(0)), b!(na!(1)))),
                                            b!(nc!(b!(na!(0)), ch!(tt)))
                                        ))
                                    ))
                                )
                            } else {
                                break Err(unexpected_atom!("*[a 9 b]", 7));
                            }
                        }
                        Atom::Direct(10) => {
                            break if let Noun::Cell(tt) = *ct!(t) {
                                if let Noun::Cell(tth) = *ch!(tt) {
                                    c!(
                                        ch!(tth),
                                        b!(nc!(
                                            b!(c!(ch!(s).clone(), ct!(tth)).tar()?),
                                            b!(c!(ch!(s), ct!(tt)).tar()?)
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
                                    Noun::Atom(_) => break c!(ch!(s), ct!(tt)).tar(),
                                    Noun::Cell(c) => {
                                        s = c!(
                                            b!(nc!(
                                                b!(c!(ch!(s).clone(), ct!(c)).tar()?),
                                                b!(c!(ch!(s), ct!(tt)).tar()?)
                                            )),
                                            b!(nc!(b!(na!(0)), b!(na!(3))))
                                        )
                                    }
                                }
                            } else {
                                break Err(unexpected_atom!("*[a 11 b]", 7));
                            }
                        }
                        Atom::Direct(n) => break Err(bad_literal!(format!("*[a {} b]", n), 6)),
                        Atom::Indirect(_) => break Err(unexpected_iatom!("*[a b c]", 6)),
                    },
                    Noun::Cell(th) => {
                        break Ok(nc!(
                            b!(c!(ch!(s).clone(), b!(Noun::Cell(th))).tar()?),
                            b!(c!(ch!(s), ct!(t)).tar()?)
                        ))
                    }
                }
            } else {
                break Err(unexpected_atom!("*[a b]", 3));
            }
        }
    }
}
