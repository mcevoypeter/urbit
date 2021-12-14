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
    fn wut(&self) {}
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
