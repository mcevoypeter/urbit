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
        a!(1 + self.value())
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
                    Atom(n) => c!(b!(na!(2 + n % 2)), b!(c!(b!(na!(n / 2)), self.t).fas()?)).fas(),
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
                    Atom(n) if 0 == n % 2 => c!(
                        b!(na!(n / 2)),
                        b!(nc!(
                            b!(nc!(t.h, b!(c!(b!(na!(n + 1)), t.t.clone()).fas()?))),
                            t.t
                        ))
                    )
                    .hax(),
                    Atom(n) => c!(
                        b!(na!(n / 2)),
                        b!(nc!(
                            b!(nc!(b!(c!(b!(na!(n - 1)), t.t.clone()).fas()?), t.h)),
                            t.t
                        ))
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
                    Noun::Atom(Atom(0)) => c!(t.t, self.h).fas(),
                    Noun::Atom(Atom(1)) => Ok(*t.t),
                    Noun::Atom(Atom(2)) => {
                        if let Noun::Cell(tt) = *t.t {
                            c!(
                                b!(c!(self.h.clone(), tt.h).tar()?),
                                b!(c!(self.h, tt.t).tar()?)
                            )
                            .tar()
                        } else {
                            Err(Error {
                                msg: "*[a 2 b] cannot be evaluated when b is an atom".to_string(),
                            })
                        }
                    }
                    Noun::Atom(Atom(3)) => match c!(self.h, t.t).tar()? {
                        Noun::Atom(a) => Ok(Noun::from_loobean(a.wut())),
                        Noun::Cell(c) => Ok(Noun::from_loobean(c.wut())),
                    },
                    Noun::Atom(Atom(4)) => {
                        if let Noun::Atom(a) = c!(self.h, t.t).tar()? {
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
                                c!(
                                    b!(c!(self.h.clone(), tt.h).tar()?),
                                    b!(c!(self.h, tt.t).tar()?)
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
                                c!(
                                    self.h.clone(),
                                    b!(c!(
                                        b!(nc!(ttt.h, ttt.t)),
                                        b!(nc!(
                                            b!(na!(0)),
                                            b!(c!(
                                                b!(nc!(b!(na!(2)), b!(na!(3)))),
                                                b!(nc!(
                                                    b!(na!(0)),
                                                    b!(c!(
                                                        self.h,
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
                            c!(b!(c!(self.h, tt.h).tar()?), tt.t).tar()
                        } else {
                            Err(Error {
                                msg: "*[a 7 b] cannot be evaluated when b is an atom".to_string(),
                            })
                        }
                    }
                    Noun::Atom(Atom(8)) => {
                        if let Noun::Cell(tt) = *t.t {
                            c!(b!(nc!(b!(c!(self.h.clone(), tt.h).tar()?), self.h)), tt.t).tar()
                        } else {
                            Err(Error {
                                msg: "*[a 8 b] cannot be evaluated when b is an atom".to_string(),
                            })
                        }
                    }
                    Noun::Atom(Atom(9)) => {
                        if let Noun::Cell(tt) = *t.t {
                            c!(
                                b!(c!(self.h, tt.t).tar()?),
                                b!(nc!(
                                    b!(na!(2)),
                                    b!(nc!(
                                        b!(nc!(b!(na!(0)), b!(na!(1)))),
                                        b!(nc!(b!(na!(0)), tt.h))
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
                        if let Noun::Cell(tt) = *t.t {
                            if let Noun::Cell(tth) = *tt.h {
                                c!(
                                    tth.h,
                                    b!(nc!(
                                        b!(c!(self.h.clone(), tth.t).tar()?),
                                        b!(c!(self.h, tt.t).tar()?)
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
                                Noun::Atom(_) => c!(self.h, tt.t).tar(),
                                Noun::Cell(c) => c!(
                                    b!(nc!(
                                        b!(c!(self.h.clone(), c.t).tar()?),
                                        b!(c!(self.h, tt.t).tar()?)
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
                        b!(c!(self.h.clone(), b!(Noun::Cell(th))).tar()?),
                        b!(c!(self.h, t.t).tar()?)
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
                                    msg: "*[a 2 b] cannot be evaluated when b is an atom"
                                        .to_string(),
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
                                        c!(
                                            b!(c!(s.h.clone(), tt.h).tar()?),
                                            b!(c!(s.h, tt.t).tar()?)
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
                                    msg: "*[a 6 b] cannot be evaluated when b is an atom"
                                        .to_string(),
                                });
                            }
                        }
                        Noun::Atom(Atom(7)) => {
                            if let Noun::Cell(tt) = *t.t {
                                s = c!(b!(c!(s.h, tt.h).tar()?), tt.t)
                            } else {
                                break Err(Error {
                                    msg: "*[a 7 b] cannot be evaluated when b is an atom"
                                        .to_string(),
                                });
                            }
                        }
                        Noun::Atom(Atom(8)) => {
                            if let Noun::Cell(tt) = *t.t {
                                s = c!(b!(nc!(b!(c!(s.h.clone(), tt.h).tar()?), s.h)), tt.t)
                            } else {
                                break Err(Error {
                                    msg: "*[a 8 b] cannot be evaluated when b is an atom"
                                        .to_string(),
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
                                    msg: "*[a 9 b] cannot be evaluated when b is an atom"
                                        .to_string(),
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
                                    msg: "*[a 10 b] cannot be evaluated when b is an atom"
                                        .to_string(),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrement() {
        // [[1 0] [0 1]] -> [1 0]
        {
            match c!(
                b!(nc!(b!(na!(1)), b!(na!(0)))),
                b!(nc!(b!(na!(0)), b!(na!(1))))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(nc!(b!(na!(1)), b!(na!(0))), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // [42 [1 0] [0 1]] -> [0 42]
        {
            match c!(
                b!(na!(42)),
                b!(nc!(
                    b!(nc!(b!(na!(1)), b!(na!(0)))),
                    b!(nc!(b!(na!(0)), b!(na!(1))))
                ))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(nc!(b!(na!(0)), b!(na!(42))), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // [4 0 1]
        let increment = b!(nc!(b!(na!(4)), b!(nc!(b!(na!(0)), b!(na!(1))))));

        // [8 [1 0] 8 [1 6 [5 [0 7] 4 0 6] [0 6] 9 2 [0 2] [4 0 6] 0 7] 9 2 0 1]
        let decrement = b!(nc!(
            b!(na!(8)),
            b!(nc!(
                b!(nc!(b!(na!(1)), b!(na!(0)))),
                b!(nc!(
                    b!(na!(8)),
                    b!(nc!(
                        b!(nc!(
                            b!(na!(1)),
                            b!(nc!(
                                b!(na!(6)),
                                b!(nc!(
                                    b!(nc!(
                                        b!(na!(5)),
                                        b!(nc!(
                                            b!(nc!(b!(na!(0)), b!(na!(7)))),
                                            b!(nc!(b!(na!(4)), b!(nc!(b!(na!(0)), b!(na!(6))))))
                                        ))
                                    )),
                                    b!(nc!(
                                        b!(nc!(b!(na!(0)), b!(na!(6)))),
                                        b!(nc!(
                                            b!(na!(9)),
                                            b!(nc!(
                                                b!(na!(2)),
                                                b!(nc!(
                                                    b!(nc!(b!(na!(0)), b!(na!(2)))),
                                                    b!(nc!(
                                                        b!(nc!(
                                                            b!(na!(4)),
                                                            b!(nc!(b!(na!(0)), b!(na!(6))))
                                                        )),
                                                        b!(nc!(b!(na!(0)), b!(na!(7))))
                                                    ))
                                                ))
                                            ))
                                        ))
                                    ))
                                ))
                            ))
                        )),
                        b!(nc!(
                            b!(na!(9)),
                            b!(nc!(b!(na!(2)), b!(nc!(b!(na!(0)), b!(na!(1))))))
                        ))
                    ))
                ))
            ))
        ));

        // *[42 decrement] -> 41
        {
            match c!(b!(na!(42)), decrement.clone()).tar() {
                Ok(res) => {
                    assert_eq!(na!(41), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[107 decrement increment] -> [106 108]
        // TODO: resolve the stack overflow that occurs when this test is run.
        {
            match c!(b!(na!(107)), b!(nc!(decrement.clone(), increment.clone()))).tar() {
                Ok(res) => {
                    assert_eq!(nc!(b!(na!(106)), b!(na!(108))), res);
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
            let t = nc!(b!(na!(98)), b!(na!(89))).into_box();
            match c!(b!(na!(1)), t.clone()).fas() {
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
            let th = b!(na!(292));
            match c!(b!(na!(2)), b!(nc!(th.clone(), b!(na!(1001))))).fas() {
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
            match c!(b!(na!(2)), b!(na!(107))).fas() {
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
            let tt = nc!(b!(na!(19)), b!(na!(95))).into_box();
            match c!(
                b!(na!(3)),
                b!(nc!(b!(nc!(b!(na!(80)), b!(na!(50)))), tt.clone()))
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
            let tht = b!(na!(16));
            match c!(
                b!(na!(5)),
                b!(nc!(b!(nc!(b!(na!(15)), tht.clone())), b!(na!(17))))
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
            let tth = b!(na!(8));
            match c!(
                b!(na!(6)),
                b!(nc!(b!(na!(4)), b!(nc!(tth.clone(), b!(na!(12))))))
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
            match c!(
                b!(na!(12)),
                b!(nc!(b!(na!(531)), b!(nc!(b!(na!(25)), b!(na!(99))))))
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
            let th = b!(na!(22));
            match c!(b!(na!(1)), b!(nc!(th.clone(), b!(na!(80))))).hax() {
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
            let th = b!(na!(11));
            let ttt = b!(na!(33));
            match c!(
                b!(na!(2)),
                b!(nc!(th.clone(), b!(nc!(b!(na!(22)), ttt.clone()))))
            )
            .hax()
            {
                Ok(res) => {
                    assert_eq!(nc!(th, ttt), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // #[3 11 [22 33]] -> [22 11]
        {
            let th = b!(na!(11));
            let tth = b!(na!(22));
            match c!(
                b!(na!(3)),
                b!(nc!(th.clone(), b!(nc!(tth.clone(), b!(na!(33))))))
            )
            .hax()
            {
                Ok(res) => {
                    assert_eq!(nc!(tth, th), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // #[4 11 [[22 33] 44]] -> [[11 33] 44]
        {
            let th = b!(na!(11));
            let ttht = b!(na!(33));
            let ttt = b!(na!(44));
            match c!(
                b!(na!(4)),
                b!(nc!(
                    th.clone(),
                    b!(nc!(b!(nc!(b!(na!(22)), ttht.clone())), ttt.clone()))
                ))
            )
            .hax()
            {
                Ok(res) => {
                    assert_eq!(nc!(b!(nc!(th, ttht)), ttt), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // #[5 11 [[22 33] 44] -> [[22 11] 44]
        {
            let th = b!(na!(11));
            let tthh = b!(na!(22));
            let ttt = b!(na!(44));
            match c!(
                b!(na!(5)),
                b!(nc!(
                    th.clone(),
                    b!(nc!(b!(nc!(tthh.clone(), b!(na!(33)))), ttt.clone()))
                ))
            )
            .hax()
            {
                Ok(res) => {
                    assert_eq!(nc!(b!(nc!(tthh, th)), ttt), res);
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
            assert_eq!(1000, a.lus().value());
        }
    }

    #[test]
    fn tar_cell() {
        // *[1 0] -> crash
        {
            match c!(b!(na!(1)), b!(na!(0))).tar() {
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
            match c!(
                b!(na!(4)),
                b!(nc!(b!(nc!(b!(na!(0)), b!(na!(0)))), b!(na!(4))))
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
            let htt = nc!(b!(na!(14)), b!(na!(15))).into_box();
            match c!(
                b!(nc!(
                    b!(nc!(b!(na!(4)), b!(na!(5)))),
                    b!(nc!(b!(na!(6)), htt.clone()))
                )),
                b!(nc!(b!(na!(0)), b!(na!(7))))
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
            let tt = nc!(b!(na!(153)), b!(na!(218))).into_box();
            match c!(b!(na!(42)), b!(nc!(b!(na!(1)), tt.clone()))).tar() {
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
            let ttttt = nc!(b!(na!(153)), b!(na!(218))).into_box();
            match c!(
                b!(na!(77)),
                b!(nc!(
                    b!(na!(2)),
                    b!(nc!(
                        b!(nc!(b!(na!(1)), b!(na!(42)))),
                        b!(nc!(b!(na!(1)), b!(nc!(b!(na!(1)), ttttt.clone()))))
                    ))
                ))
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
            match c!(
                b!(nc!(b!(na!(19)), b!(na!(20)))),
                b!(nc!(b!(na!(3)), b!(nc!(b!(na!(0)), b!(na!(1))))))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(na!(0), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[57 [4 0 1]] -> 58
        {
            match c!(
                b!(na!(57)),
                b!(nc!(b!(na!(4)), b!(nc!(b!(na!(0)), b!(na!(1))))))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(na!(58), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[12 13] 5 [1 17] [0 3]] -> 1
        {
            match c!(
                b!(nc!(b!(na!(12)), b!(na!(13)))),
                b!(nc!(
                    b!(na!(5)),
                    b!(nc!(
                        b!(nc!(b!(na!(1)), b!(na!(17)))),
                        b!(nc!(b!(na!(0)), b!(na!(3))))
                    ))
                ))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(na!(1), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [6 [1 0] [4 0 1] [1 233]]] -> 43
        {
            match c!(
                b!(na!(42)),
                b!(nc!(
                    b!(na!(6)),
                    b!(nc!(
                        b!(nc!(b!(na!(1)), b!(na!(0)))),
                        b!(nc!(
                            b!(nc!(b!(na!(4)), b!(nc!(b!(na!(0)), b!(na!(1)))))),
                            b!(nc!(b!(na!(1)), b!(na!(233))))
                        ))
                    ))
                ))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(na!(43), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [6 [1 1] [4 0 1] [1 233]]] -> 233
        {
            match c!(
                b!(na!(42)),
                b!(nc!(
                    b!(na!(6)),
                    b!(nc!(
                        b!(nc!(b!(na!(1)), b!(na!(1)))),
                        b!(nc!(
                            b!(nc!(b!(na!(4)), b!(nc!(b!(na!(0)), b!(na!(1)))))),
                            b!(nc!(b!(na!(1)), b!(na!(233))))
                        ))
                    ))
                ))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(na!(233), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [7 [4 0 1] [4 0 1]]] -> 44
        {
            match c!(
                b!(na!(42)),
                b!(nc!(
                    b!(na!(7)),
                    b!(nc!(
                        b!(nc!(b!(na!(4)), b!(nc!(b!(na!(0)), b!(na!(1)))))),
                        b!(nc!(b!(na!(4)), b!(nc!(b!(na!(0)), b!(na!(1))))))
                    ))
                ))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(na!(44), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [8 [4 0 1] [0 1]]] -> [43 42]
        {
            match c!(
                b!(na!(42)),
                b!(nc!(
                    b!(na!(8)),
                    b!(nc!(
                        b!(nc!(b!(na!(4)), b!(nc!(b!(na!(0)), b!(na!(1)))))),
                        b!(nc!(b!(na!(0)), b!(na!(1))))
                    ))
                ))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(nc!(b!(na!(43)), b!(na!(42))), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[422 [8 [4 0 1] [4 0 3]]] -> 43
        {
            match c!(
                b!(na!(42)),
                b!(nc!(
                    b!(na!(8)),
                    b!(nc!(
                        b!(nc!(b!(na!(4)), b!(nc!(b!(na!(0)), b!(na!(1)))))),
                        b!(nc!(b!(na!(4)), b!(nc!(b!(na!(0)), b!(na!(3))))))
                    ))
                ))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(na!(43), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[[0 1] 137] 9 2 [0 1]] -> [[0 1] 137]
        {
            match c!(
                b!(nc!(b!(nc!(b!(na!(0)), b!(na!(1)))), b!(na!(137)))),
                b!(nc!(
                    b!(na!(9)),
                    b!(nc!(b!(na!(2)), b!(nc!(b!(na!(0)), b!(na!(1))))))
                ))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(nc!(b!(nc!(b!(na!(0)), b!(na!(1)))), b!(na!(137))), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[[0 2] 137] 9 2 [0 1]] -> [0 2]
        {
            match c!(
                b!(nc!(b!(nc!(b!(na!(0)), b!(na!(2)))), b!(na!(137)))),
                b!(nc!(
                    b!(na!(9)),
                    b!(nc!(b!(na!(2)), b!(nc!(b!(na!(0)), b!(na!(1))))))
                ))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(nc!(b!(na!(0)), b!(na!(2))), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[[0 2] 137] 9 2 [0 1]] -> [0 2]
        {
            match c!(
                b!(nc!(b!(nc!(b!(na!(0)), b!(na!(3)))), b!(na!(137)))),
                b!(nc!(
                    b!(na!(9)),
                    b!(nc!(b!(na!(2)), b!(nc!(b!(na!(0)), b!(na!(1))))))
                ))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(na!(137), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[16 32] 10 [1 0 2] 0 3] -> 16
        {
            match c!(
                b!(nc!(b!(na!(16)), b!(na!(32)))),
                b!(nc!(
                    b!(na!(10)),
                    b!(nc!(
                        b!(nc!(b!(na!(1)), b!(nc!(b!(na!(0)), b!(na!(2)))))),
                        b!(nc!(b!(na!(0)), b!(na!(3))))
                    ))
                ))
            )
            .tar()
            {
                Ok(res) => {
                    assert_eq!(na!(16), res);
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
            assert_eq!(Loobean::Yes, c!(b!(na!(2)), b!(na!(2))).tis());
        }

        // [7 6] -> 1
        {
            assert_eq!(Loobean::No, c!(b!(na!(7)), b!(na!(6))).tis());
        }

        // [[2 7] [2 7]] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                c!(
                    b!(nc!(b!(na!(2)), b!(na!(7)))),
                    b!(nc!(b!(na!(2)), b!(na!(7))))
                )
                .tis(),
            );
        }

        // [[2 7] [2 [7 3]]] -> 1
        {
            assert_eq!(
                Loobean::No,
                c!(
                    b!(nc!(b!(na!(2)), b!(na!(7)))),
                    b!(nc!(b!(na!(2)), b!(nc!(b!(na!(7)), b!(na!(3))))))
                )
                .tis(),
            );
        }
    }

    #[test]
    fn wut_atom() {
        // ?137 -> 1
        {
            assert_eq!(Loobean::No, Atom(137).wut());
        }
    }

    #[test]
    fn wut_cell() {
        // ?[128 256] -> 0
        {
            assert_eq!(Loobean::Yes, c!(b!(na!(128)), b!(na!(256))).wut());
        }

        // ?[[512 1024] [16 32]] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                c!(
                    b!(nc!(b!(na!(512)), b!(na!(1024)))),
                    b!(nc!(b!(na!(16)), b!(na!(32))))
                )
                .wut(),
            );
        }
    }
}
