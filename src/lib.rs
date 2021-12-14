#[derive(Debug)]
#[derive(PartialEq)]
enum Loobean {
    Yes,
    No,
}

enum Noun {
    Atom(Atom),
    Cell(Cell),
}

struct Atom(u64);

struct Cell {
    head: Box<Noun>,
    tail: Box<Noun>,
}

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
    fn tis(&self) {}
}

// /
trait Fas {
    fn fas(&self) {}
}

// #
trait Hax {
    fn hax(&self) {}
}

// *
trait Tar {
    fn tar(&self) {}
}

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

impl Wut for Cell {
    fn wut(&self) -> Loobean {
        Loobean::Yes
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn wut_atom() {
        // ?137 -> 1
        let atom = Atom(137);
        assert_eq!(Loobean::No, atom.wut());
    }

    #[test]
    fn wut_cell() {
        // ?[128 256] -> 0
        let cell = Cell {
            head: Box::new(Noun::Atom(Atom(128))),
            tail: Box::new(Noun::Atom(Atom(256))),
        };
        assert_eq!(Loobean::Yes, cell.wut());

        // ?[[512 1024] [16 32]] -> 0
        let cell = Cell {
            head: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(512))),
                tail: Box::new(Noun::Atom(Atom(1024))),
            })),
            tail: Box::new(Noun::Cell(Cell {
                head: Box::new(Noun::Atom(Atom(16))),
                tail: Box::new(Noun::Atom(Atom(32))),
            })),
        };
        assert_eq!(Loobean::Yes, cell.wut());
    }

    #[test]
    fn lus_atom() {
        // +999 -> 1000
        let atom = Atom(999);
        assert_eq!(1000, atom.lus().0);
    }
}
