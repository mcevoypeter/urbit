#[derive(Debug)]
#[derive(PartialEq)]
enum Loobean {
    Yes,
    No,
}

trait Noun {}

struct Atom(u32);

struct Cell {
    head: Box<dyn Noun>,
    tail: Box<dyn Noun>,
}

// ?
trait Wut {
    fn wut(&self) -> Loobean;
}

// +
trait Lus {
    fn lus(&self) {}
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

impl Noun for Atom {}

impl Wut for Atom {
    fn wut(&self) -> Loobean {
        Loobean::No
    }
}

impl Noun for Cell {}

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
            head: Box::new(Atom(128)),
            tail: Box::new(Atom(256)),
        };
        assert_eq!(Loobean::Yes, cell.wut());

        // ?[[512 1024] [16 32]] -> 0
        let cell = Cell {
            head: Box::new(Cell {
                head: Box::new(Atom(512)),
                tail: Box::new(Atom(1024)),
            }),
            tail: Box::new(Cell {
                head: Box::new(Atom(16)),
                tail: Box::new(Atom(32)),
            }),
        };
        assert_eq!(Loobean::Yes, cell.wut());
    }
}
