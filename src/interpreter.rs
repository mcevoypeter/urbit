use crate::*;

#[cfg(all(feature = "iterative_tree", feature = "recursive_tree"))]
compile_error!(
    "feature \"iterative_tree\" and \"recursive_tree\" cannot be enabled at the same time"
);

#[cfg(feature = "iterative_tree")]
mod iterative_tree;

#[cfg(feature = "recursive_tree")]
mod recursive_tree;

/*==============================================================================
 * Nock operator trait definitions
 */

/// The ? Nock operator.
///
/// Determine if a noun is a cell or an atom.
///
/// ```console
/// ?[x] -> 0 if x is a cell
/// ?[x] -> 1 if x is an atom
/// ```
trait Wut {
    fn wut(&self) -> Loobean;
}

/// The + Nock operator.
///
/// Increment an atom.
///
/// ```console
/// +[x] -> 1 + x   if x is an atom
/// +[x] -> *crash* if x is a cell
/// ```
trait Lus {
    fn lus(self) -> Atom;
}

/// The = Nock Operator.
///
/// Determine if two nouns are equal.
///
/// ```console
/// =[x y] -> 0 if x and y are the same noun
/// =[x y] -> 1 otherwise
/// ```
trait Tis {
    fn tis(&self) -> Loobean;
}

/// The / Nock operator.
///
/// TODO(peter)
trait Fas {
    fn fas(self) -> Result<Noun, Error>;
}

/// The # Nock operator.
///
/// TODO(peter)
trait Hax {
    fn hax(self) -> Result<Noun, Error>;
}

/// The * Nock operator.
///
/// Apply the Nock interpreter function.
pub trait Tar {
    fn tar(self) -> Result<Noun, Error>;
}

/*==============================================================================
 * Interpreter-agnostic Nock operator trait implementations for Atom struct
 */

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

/*==============================================================================
 * Interpreter-agnostic Nock operator trait implementations for Cell struct
 */

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

/*==============================================================================
 * Tests
 */

#[cfg(test)]
mod tests {
    use crate::interpreter::*;

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
