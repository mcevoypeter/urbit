use crate::nock::*;

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

// ?
trait Wut {
    fn wut(&self) -> Loobean;
}

// +
trait Lus {
    fn lus(self) -> Atom;
}

// =
trait Tis {
    fn tis(&self) -> Loobean;
}

// /
trait Fas {
    fn fas(self) -> Result<Noun, Error>;
}

// #
trait Hax {
    fn hax(self) -> Result<Noun, Error>;
}

// *
trait Tar {
    fn tar(self) -> Result<Noun, Error>;
}

/*==============================================================================
 * Tests
 */

#[cfg(test)]
mod tests {
    use crate::nock::interpreters::*;

    #[test]
    fn decrement() {
        // [[1 0] [0 1]] -> [1 0]
        {
            match (Cell {
                h: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(1)).into_box(),
                    t: Noun::Atom(Atom(0)).into_box(),
                })
                .into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(0)).into_box(),
                    t: Noun::Atom(Atom(1)).into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::Cell(Cell {
                            h: Noun::Atom(Atom(1)).into_box(),
                            t: Noun::Atom(Atom(0)).into_box(),
                        }),
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
            match (Cell {
                h: Noun::Atom(Atom(42)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(1)).into_box(),
                        t: Noun::Atom(Atom(0)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(0)).into_box(),
                        t: Noun::Atom(Atom(1)).into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::Cell(Cell {
                            h: Noun::Atom(Atom(0)).into_box(),
                            t: Noun::Atom(Atom(42)).into_box(),
                        }),
                        res
                    );
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // [4 0 1]
        let increment = Noun::Cell(Cell {
            h: Noun::Atom(Atom(4)).into_box(),
            t: Noun::Cell(Cell {
                h: Noun::Atom(Atom(0)).into_box(),
                t: Noun::Atom(Atom(1)).into_box(),
            })
            .into_box(),
        })
        .into_box();

        // [8 [1 0] 8 [1 6 [5 [0 7] 4 0 6] [0 6] 9 2 [0 2] [4 0 6] 0 7] 9 2 0 1]
        let decrement = Noun::Cell(Cell {
            h: Noun::Atom(Atom(8)).into_box(),
            t: Noun::Cell(Cell {
                h: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(1)).into_box(),
                    t: Noun::Atom(Atom(0)).into_box(),
                })
                .into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(8)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(1)).into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(6)).into_box(),
                                t: Noun::Cell(Cell {
                                    h: Noun::Cell(Cell {
                                        h: Noun::Atom(Atom(5)).into_box(),
                                        t: Noun::Cell(Cell {
                                            h: Noun::Cell(Cell {
                                                h: Noun::Atom(Atom(0)).into_box(),
                                                t: Noun::Atom(Atom(7)).into_box(),
                                            })
                                            .into_box(),
                                            t: Noun::Cell(Cell {
                                                h: Noun::Atom(Atom(4)).into_box(),
                                                t: Noun::Cell(Cell {
                                                    h: Noun::Atom(Atom(0)).into_box(),
                                                    t: Noun::Atom(Atom(6)).into_box(),
                                                })
                                                .into_box(),
                                            })
                                            .into_box(),
                                        })
                                        .into_box(),
                                    })
                                    .into_box(),
                                    t: Noun::Cell(Cell {
                                        h: Noun::Cell(Cell {
                                            h: Noun::Atom(Atom(0)).into_box(),
                                            t: Noun::Atom(Atom(6)).into_box(),
                                        })
                                        .into_box(),
                                        t: Noun::Cell(Cell {
                                            h: Noun::Atom(Atom(9)).into_box(),
                                            t: Noun::Cell(Cell {
                                                h: Noun::Atom(Atom(2)).into_box(),
                                                t: Noun::Cell(Cell {
                                                    h: Noun::Cell(Cell {
                                                        h: Noun::Atom(Atom(0)).into_box(),
                                                        t: Noun::Atom(Atom(2)).into_box(),
                                                    })
                                                    .into_box(),
                                                    t: Noun::Cell(Cell {
                                                        h: Noun::Cell(Cell {
                                                            h: Noun::Atom(Atom(4)).into_box(),
                                                            t: Noun::Cell(Cell {
                                                                h: Noun::Atom(Atom(0)).into_box(),
                                                                t: Noun::Atom(Atom(6)).into_box(),
                                                            })
                                                            .into_box(),
                                                        })
                                                        .into_box(),
                                                        t: Noun::Cell(Cell {
                                                            h: Noun::Atom(Atom(0)).into_box(),
                                                            t: Noun::Atom(Atom(7)).into_box(),
                                                        })
                                                        .into_box(),
                                                    })
                                                    .into_box(),
                                                })
                                                .into_box(),
                                            })
                                            .into_box(),
                                        })
                                        .into_box(),
                                    })
                                    .into_box(),
                                })
                                .into_box(),
                            })
                            .into_box(),
                        })
                        .into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(9)).into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(2)).into_box(),
                                t: Noun::Cell(Cell {
                                    h: Noun::Atom(Atom(0)).into_box(),
                                    t: Noun::Atom(Atom(1)).into_box(),
                                })
                                .into_box(),
                            })
                            .into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            })
            .into_box(),
        })
        .into_box();

        // *[42 decrement] -> 41
        {
            match (Cell {
                h: Noun::Atom(Atom(42)).into_box(),
                t: decrement.clone(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Noun::Atom(Atom(41)), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[107 decrement increment] -> [106 108]
        // TODO: resolve the stack overflow that occurs when this test is run.
        {
            match (Cell {
                h: Noun::Atom(Atom(107)).into_box(),
                t: Noun::Cell(Cell {
                    h: decrement.clone(),
                    t: increment.clone(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::Cell(Cell {
                            h: Noun::Atom(Atom(106)).into_box(),
                            t: Noun::Atom(Atom(108)).into_box(),
                        }),
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
            let t = Noun::Cell(Cell {
                h: Noun::Atom(Atom(98)).into_box(),
                t: Noun::Atom(Atom(89)).into_box(),
            })
            .into_box();
            match (Cell {
                h: Noun::Atom(Atom(1)).into_box(),
                t: t.clone(),
            }
            .fas())
            {
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
            let th = Noun::Atom(Atom(292)).into_box();
            match (Cell {
                h: Noun::Atom(Atom(2)).into_box(),
                t: Noun::Cell(Cell {
                    h: th.clone(),
                    t: Noun::Atom(Atom(1001)).into_box(),
                })
                .into_box(),
            }
            .fas())
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
            match (Cell {
                h: Noun::Atom(Atom(2)).into_box(),
                t: Noun::Atom(Atom(107)).into_box(),
            }
            .fas())
            {
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
            let tt = Noun::Cell(Cell {
                h: Noun::Atom(Atom(19)).into_box(),
                t: Noun::Atom(Atom(95)).into_box(),
            })
            .into_box();
            match (Cell {
                h: Noun::Atom(Atom(3)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(80)).into_box(),
                        t: Noun::Atom(Atom(50)).into_box(),
                    })
                    .into_box(),
                    t: tt.clone(),
                })
                .into_box(),
            }
            .fas())
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
            let tht = Noun::Atom(Atom(16)).into_box();
            match (Cell {
                h: Noun::Atom(Atom(5)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(15)).into_box(),
                        t: tht.clone(),
                    })
                    .into_box(),
                    t: Noun::Atom(Atom(17)).into_box(),
                })
                .into_box(),
            }
            .fas())
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
            let tth = Noun::Atom(Atom(8)).into_box();
            match (Cell {
                h: Noun::Atom(Atom(6)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(4)).into_box(),
                    t: Noun::Cell(Cell {
                        h: tth.clone(),
                        t: Noun::Atom(Atom(12)).into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .fas())
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
            match (Cell {
                h: Noun::Atom(Atom(12)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(531)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(25)).into_box(),
                        t: Noun::Atom(Atom(99)).into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .fas())
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
            let th = Noun::Atom(Atom(22)).into_box();
            match (Cell {
                h: Noun::Atom(Atom(1)).into_box(),
                t: Noun::Cell(Cell {
                    h: th.clone(),
                    t: Noun::Atom(Atom(80)).into_box(),
                })
                .into_box(),
            }
            .hax())
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
            let th = Noun::Atom(Atom(11)).into_box();
            let ttt = Noun::Atom(Atom(33)).into_box();
            match (Cell {
                h: Noun::Atom(Atom(2)).into_box(),
                t: Noun::Cell(Cell {
                    h: th.clone(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(22)).into_box(),
                        t: ttt.clone(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .hax())
            {
                Ok(res) => {
                    assert_eq!(Noun::Cell(Cell { h: th, t: ttt }), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // #[3 11 [22 33]] -> [22 11]
        {
            let th = Noun::Atom(Atom(11)).into_box();
            let tth = Noun::Atom(Atom(22)).into_box();
            match (Cell {
                h: Noun::Atom(Atom(3)).into_box(),
                t: Noun::Cell(Cell {
                    h: th.clone(),
                    t: Noun::Cell(Cell {
                        h: tth.clone(),
                        t: Noun::Atom(Atom(33)).into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .hax())
            {
                Ok(res) => {
                    assert_eq!(Noun::Cell(Cell { h: tth, t: th }), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // #[4 11 [[22 33] 44]] -> [[11 33] 44]
        {
            let th = Noun::Atom(Atom(11)).into_box();
            let ttht = Noun::Atom(Atom(33)).into_box();
            let ttt = Noun::Atom(Atom(44)).into_box();
            match (Cell {
                h: Noun::Atom(Atom(4)).into_box(),
                t: Noun::Cell(Cell {
                    h: th.clone(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(22)).into_box(),
                            t: ttht.clone(),
                        })
                        .into_box(),
                        t: ttt.clone(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .hax())
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::Cell(Cell {
                            h: Noun::Cell(Cell { h: th, t: ttht }).into_box(),
                            t: ttt,
                        }),
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
            let th = Noun::Atom(Atom(11)).into_box();
            let tthh = Noun::Atom(Atom(22)).into_box();
            let ttt = Noun::Atom(Atom(44)).into_box();
            match (Cell {
                h: Noun::Atom(Atom(5)).into_box(),
                t: Noun::Cell(Cell {
                    h: th.clone(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: tthh.clone(),
                            t: Noun::Atom(Atom(33)).into_box(),
                        })
                        .into_box(),
                        t: ttt.clone(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .hax())
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::Cell(Cell {
                            h: Noun::Cell(Cell { h: tthh, t: th }).into_box(),
                            t: ttt,
                        }),
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
            match (Cell {
                h: Noun::Atom(Atom(1)).into_box(),
                t: Noun::Atom(Atom(0)).into_box(),
            }
            .tar())
            {
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
            match (Cell {
                h: Noun::Atom(Atom(4)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(0)).into_box(),
                        t: Noun::Atom(Atom(0)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Atom(Atom(4)).into_box(),
                })
                .into_box(),
            }
            .tar())
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
            let htt = Noun::Cell(Cell {
                h: Noun::Atom(Atom(14)).into_box(),
                t: Noun::Atom(Atom(15)).into_box(),
            })
            .into_box();
            match (Cell {
                h: Noun::Cell(Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(4)).into_box(),
                        t: Noun::Atom(Atom(5)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(6)).into_box(),
                        t: htt.clone(),
                    })
                    .into_box(),
                })
                .into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(0)).into_box(),
                    t: Noun::Atom(Atom(7)).into_box(),
                })
                .into_box(),
            }
            .tar())
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
            let tt = Noun::Cell(Cell {
                h: Noun::Atom(Atom(153)).into_box(),
                t: Noun::Atom(Atom(218)).into_box(),
            })
            .into_box();
            match (Cell {
                h: Noun::Atom(Atom(42)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(1)).into_box(),
                    t: tt.clone(),
                })
                .into_box(),
            }
            .tar())
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
            let ttttt = Noun::Cell(Cell {
                h: Noun::Atom(Atom(153)).into_box(),
                t: Noun::Atom(Atom(218)).into_box(),
            })
            .into_box();
            match (Cell {
                h: Noun::Atom(Atom(77)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(2)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(1)).into_box(),
                            t: Noun::Atom(Atom(42)).into_box(),
                        })
                        .into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(1)).into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(1)).into_box(),
                                t: ttttt.clone(),
                            })
                            .into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
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
            match (Cell {
                h: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(19)).into_box(),
                    t: Noun::Atom(Atom(20)).into_box(),
                })
                .into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(3)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(0)).into_box(),
                        t: Noun::Atom(Atom(1)).into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Noun::Atom(Atom(0)), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[57 [4 0 1]] -> 58
        {
            match (Cell {
                h: Noun::Atom(Atom(57)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(4)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(0)).into_box(),
                        t: Noun::Atom(Atom(1)).into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Noun::Atom(Atom(58)), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[12 13] 5 [1 17] [0 3]] -> 1
        {
            match (Cell {
                h: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(12)).into_box(),
                    t: Noun::Atom(Atom(13)).into_box(),
                })
                .into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(5)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(1)).into_box(),
                            t: Noun::Atom(Atom(17)).into_box(),
                        })
                        .into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(0)).into_box(),
                            t: Noun::Atom(Atom(3)).into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Noun::Atom(Atom(1)), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [6 [1 0] [4 0 1] [1 233]]] -> 43
        {
            match (Cell {
                h: Noun::Atom(Atom(42)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(6)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(1)).into_box(),
                            t: Noun::Atom(Atom(0)).into_box(),
                        })
                        .into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(4)).into_box(),
                                t: Noun::Cell(Cell {
                                    h: Noun::Atom(Atom(0)).into_box(),
                                    t: Noun::Atom(Atom(1)).into_box(),
                                })
                                .into_box(),
                            })
                            .into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(1)).into_box(),
                                t: Noun::Atom(Atom(233)).into_box(),
                            })
                            .into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Noun::Atom(Atom(43)), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [6 [1 1] [4 0 1] [1 233]]] -> 233
        {
            match (Cell {
                h: Noun::Atom(Atom(42)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(6)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(1)).into_box(),
                            t: Noun::Atom(Atom(1)).into_box(),
                        })
                        .into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(4)).into_box(),
                                t: Noun::Cell(Cell {
                                    h: Noun::Atom(Atom(0)).into_box(),
                                    t: Noun::Atom(Atom(1)).into_box(),
                                })
                                .into_box(),
                            })
                            .into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(1)).into_box(),
                                t: Noun::Atom(Atom(233)).into_box(),
                            })
                            .into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Noun::Atom(Atom(233)), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [7 [4 0 1] [4 0 1]]] -> 44
        {
            match (Cell {
                h: Noun::Atom(Atom(42)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(7)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(4)).into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(0)).into_box(),
                                t: Noun::Atom(Atom(1)).into_box(),
                            })
                            .into_box(),
                        })
                        .into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(4)).into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(0)).into_box(),
                                t: Noun::Atom(Atom(1)).into_box(),
                            })
                            .into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Noun::Atom(Atom(44)), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[42 [8 [4 0 1] [0 1]]] -> [43 42]
        {
            match (Cell {
                h: Noun::Atom(Atom(42)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(8)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(4)).into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(0)).into_box(),
                                t: Noun::Atom(Atom(1)).into_box(),
                            })
                            .into_box(),
                        })
                        .into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(0)).into_box(),
                            t: Noun::Atom(Atom(1)).into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::Cell(Cell {
                            h: Noun::Atom(Atom(43)).into_box(),
                            t: Noun::Atom(Atom(42)).into_box(),
                        }),
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
            match (Cell {
                h: Noun::Atom(Atom(42)).into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(8)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(4)).into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(0)).into_box(),
                                t: Noun::Atom(Atom(1)).into_box(),
                            })
                            .into_box(),
                        })
                        .into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(4)).into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(0)).into_box(),
                                t: Noun::Atom(Atom(3)).into_box(),
                            })
                            .into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Noun::Atom(Atom(43)), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[[0 1] 137] 9 2 [0 1]] -> [[0 1] 137]
        {
            match (Cell {
                h: Noun::Cell(Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(0)).into_box(),
                        t: Noun::Atom(Atom(1)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Atom(Atom(137)).into_box(),
                })
                .into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(9)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(2)).into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(0)).into_box(),
                            t: Noun::Atom(Atom(1)).into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::Cell(Cell {
                            h: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(0)).into_box(),
                                t: Noun::Atom(Atom(1)).into_box(),
                            })
                            .into_box(),
                            t: Noun::Atom(Atom(137)).into_box(),
                        }),
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
            match (Cell {
                h: Noun::Cell(Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(0)).into_box(),
                        t: Noun::Atom(Atom(2)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Atom(Atom(137)).into_box(),
                })
                .into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(9)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(2)).into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(0)).into_box(),
                            t: Noun::Atom(Atom(1)).into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(
                        Noun::Cell(Cell {
                            h: Noun::Atom(Atom(0)).into_box(),
                            t: Noun::Atom(Atom(2)).into_box(),
                        }),
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
            match (Cell {
                h: Noun::Cell(Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(0)).into_box(),
                        t: Noun::Atom(Atom(3)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Atom(Atom(137)).into_box(),
                })
                .into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(9)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(2)).into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(0)).into_box(),
                            t: Noun::Atom(Atom(1)).into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Noun::Atom(Atom(137)), res);
                }
                Err(err) => {
                    assert!(false, "Unexpected failure: {}.", err.msg);
                }
            }
        }

        // *[[16 32] 10 [1 0 2] 0 3] -> 16
        {
            match (Cell {
                h: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(16)).into_box(),
                    t: Noun::Atom(Atom(32)).into_box(),
                })
                .into_box(),
                t: Noun::Cell(Cell {
                    h: Noun::Atom(Atom(10)).into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(1)).into_box(),
                            t: Noun::Cell(Cell {
                                h: Noun::Atom(Atom(0)).into_box(),
                                t: Noun::Atom(Atom(2)).into_box(),
                            })
                            .into_box(),
                        })
                        .into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(0)).into_box(),
                            t: Noun::Atom(Atom(3)).into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                })
                .into_box(),
            }
            .tar())
            {
                Ok(res) => {
                    assert_eq!(Noun::Atom(Atom(16)), res);
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
                Cell {
                    h: Noun::Atom(Atom(2)).into_box(),
                    t: Noun::Atom(Atom(2)).into_box(),
                }
                .tis(),
            );
        }

        // [7 6] -> 1
        {
            assert_eq!(
                Loobean::No,
                Cell {
                    h: Noun::Atom(Atom(7)).into_box(),
                    t: Noun::Atom(Atom(6)).into_box(),
                }
                .tis(),
            );
        }

        // [[2 7] [2 7]] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(2)).into_box(),
                        t: Noun::Atom(Atom(7)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(2)).into_box(),
                        t: Noun::Atom(Atom(7)).into_box(),
                    })
                    .into_box(),
                }
                .tis(),
            );
        }

        // [[2 7] [2 [7 3]]] -> 1
        {
            assert_eq!(
                Loobean::No,
                Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(2)).into_box(),
                        t: Noun::Atom(Atom(7)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(2)).into_box(),
                        t: Noun::Cell(Cell {
                            h: Noun::Atom(Atom(7)).into_box(),
                            t: Noun::Atom(Atom(3)).into_box(),
                        })
                        .into_box(),
                    })
                    .into_box(),
                }
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
                Cell {
                    h: Noun::Atom(Atom(128)).into_box(),
                    t: Noun::Atom(Atom(256)).into_box(),
                }
                .wut(),
            );
        }

        // ?[[512 1024] [16 32]] -> 0
        {
            assert_eq!(
                Loobean::Yes,
                Cell {
                    h: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(512)).into_box(),
                        t: Noun::Atom(Atom(1024)).into_box(),
                    })
                    .into_box(),
                    t: Noun::Cell(Cell {
                        h: Noun::Atom(Atom(16)).into_box(),
                        t: Noun::Atom(Atom(32)).into_box(),
                    })
                    .into_box(),
                }
                .wut(),
            );
        }
    }
}
