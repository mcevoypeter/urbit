use crate::noun::Noun;

/// A pair of heap-allocated nouns.
#[derive(Debug)]
pub struct Cell {
    pub head: Box<Noun>,
    pub tail: Box<Noun>,
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell {
            head: self.head.clone(),
            tail: self.tail.clone(),
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        if let (Noun::Atom(lh_h), Noun::Atom(rh_h)) = (&*self.head, &*other.head) {
            lh_h == rh_h && *self.tail == *other.tail
        } else if let (Noun::Cell(lh_h), Noun::Cell(rh_h)) = (&*self.head, &*other.head) {
            Self::eq(lh_h, rh_h) && *self.tail == *other.tail
        } else {
            false
        }
    }
}

/// Create a cell.
#[macro_export]
macro_rules! c {
    ($h:expr, $t:expr) => {
        crate::cell::Cell { head: $h, tail: $t }
    };
}

/// Get the head of a cell.
#[macro_export]
macro_rules! ch {
    ($c:expr) => {
        $c.head
    };
}

/// Get the tail of a cell.
#[macro_export]
macro_rules! ct {
    ($c:expr) => {
        $c.tail
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
