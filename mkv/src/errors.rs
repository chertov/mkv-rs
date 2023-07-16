use super::{EbmlId, ElementSize};

/// A possible error when parsing a Matroska file
#[derive(thiserror::Error, Debug)]
pub enum MatroskaError {
    #[error("I/O error: {0:?}")]
    Io(std::io::Error),
    #[error("Error decoding a UTF-8 string: {0:?}")]
    UTF8(std::string::FromUtf8Error),
    #[error("Invalid element ID, unknown value (0x{0:02X?})")]
    InvalidID(u64),
    #[error("Invalid element size. Element Id '{0:?}', size '{1:?}'")]
    InvalidSize(EbmlId, ElementSize),
    #[error("var int value {0} more than VINT_MAX value")]
    InvalidVarIntMoreThanVintMax(u64),
    #[error("invalid variable integer")]
    InvalidVarInt,
    #[error("invalid unsigned integer")]
    InvalidUint,
    #[error("invalid floating point value")]
    InvalidFloat,
    #[error("invalid date value")]
    InvalidDate,
    #[error("Invalid seek head entry (0x{id:02X?})")]
    InvalidSeekHead { id: u32, },
}

impl From<std::io::Error> for MatroskaError {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
