use super::*;

#[cfg(feature = "iterative_tree")]
mod iterative_tree;
#[cfg(feature = "recursive_tree")]
mod recursive_tree;

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
        a!(1 + av!(self))
    }
}

impl Wut for Cell {
    fn wut(&self) -> Loobean {
        Loobean::Yes
    }
}

impl Tis for Cell {
    fn tis(&self) -> Loobean {
        Loobean::from_boolean(ch!(self) == ct!(self))
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
                }
            }
        }

        // *[107 decrement increment] -> [106 108]
        // Note: this test overflows the stack when run using the interpreter defined in
        // recursive_tree.rs.
        {
            match c!(b!(na!(107)), b!(nc!(decrement.clone(), increment.clone()))).tar() {
                Ok(res) => {
                    assert_eq!(nc!(b!(na!(106)), b!(na!(108))), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
                }
            }
        }
    }

    #[test]
    fn lus_atom() {
        // +999 -> 1000
        {
            let a = a!(999);
            assert_eq!(1000, av!(a.lus()));
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
                    assert!(false, "Unexpected failure: {}.", err);
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
            assert_eq!(Loobean::No, a!(137).wut());
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
