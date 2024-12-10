mod de;
mod ser;

use std::error::Error as StdError;
use std::fmt;

use de::Decoder;
use ser::Encoder;

pub fn serialize<T>(value: &T) -> Result<Vec<u8>, Box<Error>>
where
    T: serde::ser::Serialize,
{
    let mut buf = Vec::new();
    serialize_into(&mut buf, value)?;
    Ok(buf)
}

pub fn serialize_into<T>(buf: &mut Vec<u8>, value: &T) -> Result<(), Box<Error>>
where
    T: serde::ser::Serialize,
{
    value.serialize(Encoder::new(buf))?;
    Ok(())
}

pub fn deserialize<'de, T>(buf: &'de [u8]) -> Result<T, Box<Error>>
where
    T: serde::de::Deserialize<'de>,
{
    T::deserialize(&mut Decoder::new(buf))
}

#[derive(Debug)]
pub enum Error {
    MissingData,
    NotSupported,
    InvalidBool,
    InvalidChar,
    InvalidStr,
    InvalidOption,
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingData => fmt.write_str("missing data"),
            Self::NotSupported => fmt.write_str("not supported"),
            Self::InvalidBool => fmt.write_str("invalid bool"),
            Self::InvalidChar => fmt.write_str("invalid char"),
            Self::InvalidStr => fmt.write_str("invalid str"),
            Self::InvalidOption => fmt.write_str("invalid option"),
            Self::Custom(msg) => write!(fmt, "custom: {msg}"),
        }
    }
}

impl StdError for Error {}

impl serde::ser::Error for Box<Error> {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Box::new(Error::Custom(msg.to_string()))
    }
}

impl serde::de::Error for Box<Error> {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Box::new(Error::Custom(msg.to_string()))
    }
}
