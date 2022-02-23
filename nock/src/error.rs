use std::{error, fmt};

/// A Nock-specific error encapsulating an informative error message.
#[derive(Debug)]
pub enum Error {
    BadLiteral(String, u64),
    UnexpectedAtom(String, u64),
    UnexpectedIndirectAtom(String, u64),
    UnexpectedCell(String, u64),
}

/// Create instance of Error::BadLiteral.
#[macro_export]
macro_rules! bad_literal {
    ($expr:expr, $axis:expr) => {
        crate::error::Error::BadLiteral($expr.to_string(), $axis)
    };
}

/// Create instance of Error::UnexpectedAtom.
#[macro_export]
macro_rules! unexpected_atom {
    ($expr:expr, $axis:expr) => {
        crate::error::Error::UnexpectedAtom($expr.to_string(), $axis)
    };
}

/// Create instance of Error::UnexpectedIndirectAtom.
#[macro_export]
macro_rules! unexpected_iatom {
    ($expr:expr, $axis:expr) => {
        crate::error::Error::UnexpectedIndirectAtom($expr.to_string(), $axis)
    };
}

/// Create instance of Error::UnexpectedCell.
#[macro_export]
macro_rules! unexpected_cell {
    ($expr:expr, $axis:expr) => {
        crate::error::Error::UnexpectedCell($expr.to_string(), $axis)
    };
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadLiteral(expr, axis) => {
                write!(f, "encountered bad literal at axis {} of {}", axis, expr)
            }
            Error::UnexpectedAtom(expr, axis) => {
                write!(
                    f,
                    "encountered unexpected atom at axis {} of {}",
                    axis, expr
                )
            }
            Error::UnexpectedIndirectAtom(expr, axis) => {
                write!(
                    f,
                    "encountered unexpected indirect atom at axis {} of {}",
                    axis, expr
                )
            }
            Error::UnexpectedCell(expr, axis) => {
                write!(
                    f,
                    "encountered unexpected cell at axis {} of {}",
                    axis, expr
                )
            }
        }
    }
}

impl error::Error for Error {}
