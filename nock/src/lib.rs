#[macro_use]
pub mod atom;
#[macro_use]
pub mod cell;
pub mod error;
pub mod interpreters;
pub mod loobean;
pub mod noun;

#[global_allocator]
static GLOBAL: loom::Loom = loom::Loom;

/// Box::new($e)
#[macro_export]
macro_rules! b {
    ($e:expr) => {
        Box::new($e)
    };
}
