//! 
//! Potential errors that can occur when reading or writing WebM data.
//!

use std::fmt;
use std::error::Error;

pub use ebml_iterable::error::TagIteratorError;
pub use ebml_iterable::error::TagWriterError;

///
/// Errors that can occur when coercing WebM data into structs.
///
#[derive(Debug)]
pub enum WebmCoercionError {

    ///
    /// An error when coercing raw Block data into a [`super::matroska_spec::Block`] struct.
    ///
    BlockCoercionError(String),

    ///
    /// An error when coercing raw SimpleBlock data into a [`super::matroska_spec::SimpleBlock`] struct.
    ///
    SimpleBlockCoercionError(String),
}

impl fmt::Display for WebmCoercionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebmCoercionError::BlockCoercionError(msg) => write!(f, "{}", msg),
            WebmCoercionError::SimpleBlockCoercionError(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for WebmCoercionError {}