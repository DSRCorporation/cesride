pub type BoxedError = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, BoxedError>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("matter error: {0}")]
    Matter(String),
    #[error("empty material: {0}")]
    EmptyMaterial(String),
    #[error("decode error: {0}")]
    Decode(String),
    #[error("unexpected code error: {0}")]
    UnexpectedCode(String),
    #[error("unexpected count code error: {0}")]
    UnexpectedCountCode(String),
    #[error("unexpected op code error: {0}")]
    UnexpectedOpCode(String),
    #[error("invalid variable size: {0}")]
    InvalidVarSize(String),
    #[error("invalid variable raw size: {0}")]
    InvalidVarRawSize(String),
    #[error("invalid variable index: {0}")]
    InvalidVarIndex(String),
    #[error("invalid code size: {0}")]
    InvalidCodeSize(String),
    #[error("invalid base64 character: {0}")]
    InvalidBase64Character(char),
    #[error("invalid base64 index: {0}")]
    InvalidBase64Index(u8),
    #[error("shortage: {0}")]
    Shortage(String),
    #[error("empty qb64")]
    EmptyQb64(),
    #[error("unknown sizage: {0}")]
    UnknownSizage(String),
    #[error("unknown hardage: {0}")]
    UnknownHardage(String),
    #[error("unknown bardage: {0}")]
    UnknownBardage(String),
    #[error("variable size codes not supported")]
    UnsupportedSize(),
    #[error("need {0} more characters")]
    TooSmall(usize),
    #[error("prepad error")]
    Prepad(),
    #[error("non-zeroed prepad bits")]
    NonZeroedPrepad(),
    #[error("non-zeroed lead byte")]
    NonZeroedLeadByte(),
    #[error("non-zeroed lead bytes")]
    NonZeroedLeadBytes(),
    #[error("non-zeroed pad bits")]
    NonZeroedPadBits(),
    #[error("parsing error: {0}")]
    Parsing(String),
    #[error("error parsing qb64: {0}")]
    ParseQb64(String),
    #[error("error parsing qb2: {0}")]
    ParseQb2(String),
    #[error("conversion error: {0}")]
    Conversion(String),
}

macro_rules! err {
    ($e:expr) => {
        Err(Box::new($e))
    };
}

pub(crate) use err;

#[cfg(test)]
mod test {
    use super::{Error, Result};

    fn explode() -> Result<()> {
        return err!(Error::Prepad());
    }

    #[test]
    fn err() {
        assert!(explode().is_err());
    }
}
