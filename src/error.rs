use alloc::boxed::Box;
use core::fmt;

use crate::Diagnostic;

/**
Error enum for miette. Used by certain operations in the protocol.
*/
#[derive(Debug, thiserror::Error)]
pub enum MietteError {
    /// Wrapper around [`std::io::Error`]. This is returned when something went
    /// wrong while reading a [`SourceCode`](crate::SourceCode).
    #[cfg(feature = "std")]
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Wrapper around [`std::io::Error`]. This is returned when something went
    /// wrong while reading a [`SourceCode`](crate::SourceCode).
    #[cfg(not(feature = "std"))]
    #[error("i/o error: {0}")]
    IoError(alloc::string::String),

    /// Returned when a [`SourceSpan`](crate::SourceSpan) extends beyond the
    /// bounds of a given [`SourceCode`](crate::SourceCode).
    #[error("The given offset is outside the bounds of its Source")]
    OutOfBounds,
}

#[cfg(not(feature = "std"))]
impl From<alloc::string::String> for MietteError {
    fn from(message: alloc::string::String) -> Self {
        Self::IoError(message)
    }
}

impl Diagnostic for MietteError {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            MietteError::IoError(_) => Some(Box::new("miette::io_error")),
            MietteError::OutOfBounds => Some(Box::new("miette::span_out_of_bounds")),
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            MietteError::IoError(_) => None,
            MietteError::OutOfBounds => Some(Box::new(
                "Double-check your spans. Do you have an off-by-one error?",
            )),
        }
    }

    fn url<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        let crate_version = env!("CARGO_PKG_VERSION");
        let variant = match self {
            MietteError::IoError(_) => "#variant.IoError",
            MietteError::OutOfBounds => "#variant.OutOfBounds",
        };
        Some(Box::new(format!(
            "https://docs.rs/miette/{}/miette/enum.MietteError.html{}",
            crate_version, variant,
        )))
    }
}
