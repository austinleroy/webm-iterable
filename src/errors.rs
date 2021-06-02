use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum WebmError {
    BlockCoercionError(String),
    SimpleBlockCoercionError(String),
}

impl fmt::Display for WebmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebmError::BlockCoercionError(msg) => write!(f, "{}", msg),
            WebmError::SimpleBlockCoercionError(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for WebmError {}