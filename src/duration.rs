// wrapper for std::time::Duration with impl for Display and FromStr

use std::{error::Error, num::{ParseFloatError, ParseIntError}, str::FromStr};

#[derive(Debug, Default, Clone, Copy)]
pub struct Duration(pub std::time::Duration);

impl std::fmt::Display for Duration {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Debug;
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct ParseDurationError {
    cause: Option<Box<dyn Error + Send + Sync>>,
}

impl ParseDurationError {
    #[inline]
    pub fn new() -> Self {
        Self { cause: None }
    }

    #[inline]
    pub fn with_cause(cause: Box<dyn Error + Send + Sync>) -> Self {
        Self { cause: Some(cause) }
    }
}

impl std::fmt::Display for ParseDurationError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(cause) = &self.cause {
            write!(f, "Invalid duration syntax: {}", cause)
        } else {
            "Invalid duration syntax".fmt(f)
        }
    }
}

impl Error for ParseDurationError {
    #[inline]
    fn cause(&self) -> Option<&dyn Error> {
        let Some(cause) = &self.cause else {
            return None;
        };

        Some(cause.as_ref())
    }
}

impl From<ParseFloatError> for ParseDurationError {
    #[inline]
    fn from(value: ParseFloatError) -> Self {
        Self::with_cause(Box::new(value))
    }
}

impl From<ParseIntError> for ParseDurationError {
    #[inline]
    fn from(value: ParseIntError) -> Self {
        Self::with_cause(Box::new(value))
    }
}

impl FromStr for Duration {
    type Err = ParseDurationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();

        let index = value.find(|ch: char| ch != '.' && !ch.is_numeric()).unwrap_or(value.len());

        let unit = value[index..].trim();
        let value = value[..index].trim();

        if unit.is_empty() || unit.eq_ignore_ascii_case("s") || unit.eq_ignore_ascii_case("sec") {
            let value = value.parse()?;
            Ok(Duration(std::time::Duration::from_secs_f64(value)))
        } else if unit.eq_ignore_ascii_case("ms") || unit.eq_ignore_ascii_case("msec") {
            let value = value.parse()?;
            Ok(Duration(std::time::Duration::from_millis(value)))
        } else if unit.eq_ignore_ascii_case("ns") || unit.eq_ignore_ascii_case("nsec") {
            let value = value.parse()?;
            Ok(Duration(std::time::Duration::from_nanos(value)))
        } else if unit.eq_ignore_ascii_case("m") || unit.eq_ignore_ascii_case("min") {
            let value: f64 = value.parse()?;
            Ok(Duration(std::time::Duration::from_secs_f64(60.0 * value)))
        } else if unit.eq_ignore_ascii_case("h") || unit.eq_ignore_ascii_case("hour") {
            let value: f64 = value.parse()?;
            Ok(Duration(std::time::Duration::from_secs_f64(60.0 * 60.0 * value)))
        } else if unit.eq_ignore_ascii_case("d") || unit.eq_ignore_ascii_case("day") {
            let value: f64 = value.parse()?;
            Ok(Duration(std::time::Duration::from_secs_f64(24.0 * 60.0 * 60.0 * value)))
        } else {
            Err(ParseDurationError::new())
        }
    }
}
