use crate::noun::Noun;

/// A pair of heap-allocated nouns.
#[derive(Debug)]
pub struct Cell {
    pub h: Box<Noun>,
    pub t: Box<Noun>,
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell {
            h: self.h.clone(),
            t: self.t.clone(),
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        if let (Noun::Atom(lh_h), Noun::Atom(rh_h)) = (&*self.h, &*other.h) {
            lh_h == rh_h && *self.t == *other.t
        } else if let (Noun::Cell(lh_h), Noun::Cell(rh_h)) = (&*self.h, &*other.h) {
            Self::eq(lh_h, rh_h) && *self.t == *other.t
        } else {
            false
        }
    }
}

/// Create a cell.
#[macro_export]
macro_rules! c {
    ($h:expr, $t:expr) => {
        crate::cell::Cell { h: $h, t: $t }
    };
}

#[cfg(test)]
mod tests {
    use crate::{b, na, nc};

    #[test]
    fn clone() {
        // Clone [8 808].
        {
            let c = c!(b!(na!(8)), b!(na!(808)));
            assert_eq!(c, c.clone());
        }
    }

    #[test]
    fn partialeq() {
        // [71 109] == [71 109]
        {
            assert_eq!(c!(b!(na!(71)), b!(na!(109))), c!(b!(na!(71)), b!(na!(109))));
        }

        // [71 109] != [109 71]
        {
            assert_ne!(c!(b!(na!(71)), b!(na!(109))), c!(b!(na!(109)), b!(na!(71))));
        }

        // [11 [12 13]] == [11 [12 13]]
        {
            assert_eq!(
                nc!(b!(na!(11)), b!(nc!(b!(na!(12)), b!(na!(13))))),
                nc!(b!(na!(11)), b!(nc!(b!(na!(12)), b!(na!(13)))))
            );
        }

        // [11 [12 13]] != [11 [13 12]]
        {
            assert_ne!(
                nc!(b!(na!(11)), b!(nc!(b!(na!(12)), b!(na!(13))))),
                nc!(b!(na!(11)), b!(nc!(b!(na!(13)), b!(na!(12)))))
            );
        }
    }
}
