
use std::{error::Error, fmt::Display, num::ParseIntError, ops::{Index, IndexMut}, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Rgb(pub [u8; 3]);

pub const BLACK:      Rgb = Rgb([  0,   0,   0]);
pub const WHITE:      Rgb = Rgb([255, 255, 255]);
pub const TRANS_PINK: Rgb = Rgb([245, 168, 184]);
pub const TRANS_BLUE: Rgb = Rgb([ 91, 207, 249]);
pub const BROWN:      Rgb = Rgb([149,  85,  23]);
pub const RED:        Rgb = Rgb([227,  32,  32]);
pub const ORANGE:     Rgb = Rgb([245, 136,  23]);
pub const YELLOW:     Rgb = Rgb([240, 229,  37]);
pub const GREEN:      Rgb = Rgb([121, 184,  43]);
pub const BLUE:       Rgb = Rgb([ 45,  89, 163]);
pub const PURPLE:     Rgb = Rgb([109,  35, 128]);

impl Index<usize> for Rgb {
    type Output = u8;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Rgb {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.0[index]
    }
}

impl From<[u8; 3]> for Rgb {
    #[inline]
    fn from(value: [u8; 3]) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct ParseRgbError {
    cause: Option<Box<dyn Error + Send + Sync>>,
}

impl ParseRgbError {
    #[inline]
    pub fn new() -> Self {
        Self { cause: None }
    }

    #[inline]
    pub fn with_cause(cause: Box<dyn Error + Send + Sync>) -> Self {
        Self { cause: Some(cause) }
    }
}

impl std::fmt::Display for ParseRgbError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(cause) = &self.cause {
            write!(f, "Invalid RGB color syntax: {}", cause)
        } else {
            "Invalid RGB color syntax".fmt(f)
        }
    }
}

impl Error for ParseRgbError {
    #[inline]
    fn cause(&self) -> Option<&dyn Error> {
        let Some(cause) = &self.cause else {
            return None;
        };

        Some(cause.as_ref())
    }
}

impl From<ParseIntError> for ParseRgbError {
    #[inline]
    fn from(value: ParseIntError) -> Self {
        Self::with_cause(Box::new(value))
    }
}

impl FromStr for Rgb {
    type Err = ParseRgbError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();

        if value.starts_with("#") {
            if value.len() != 7 {
                return Err(ParseRgbError::new());
            }

            let r = u8::from_str_radix(&value[1..3], 16)?;
            let g = u8::from_str_radix(&value[3..5], 16)?;
            let b = u8::from_str_radix(&value[5..7], 16)?;

            return Ok(Rgb([r, g, b]));
        } else if value.eq_ignore_ascii_case("white") {
            return Ok(WHITE);
        } else if value.eq_ignore_ascii_case("black") {
            return Ok(BLACK);
        } else if value.eq_ignore_ascii_case("trans-pink") {
            return Ok(TRANS_PINK);
        } else if value.eq_ignore_ascii_case("trans-blue") {
            return Ok(TRANS_BLUE);
        } else if value.eq_ignore_ascii_case("brown") {
            return Ok(BROWN);
        } else if value.eq_ignore_ascii_case("red") {
            return Ok(RED);
        } else if value.eq_ignore_ascii_case("orange") {
            return Ok(ORANGE);
        } else if value.eq_ignore_ascii_case("yellow") {
            return Ok(YELLOW);
        } else if value.eq_ignore_ascii_case("green") {
            return Ok(GREEN);
        } else if value.eq_ignore_ascii_case("blue") {
            return Ok(BLUE);
        } else if value.eq_ignore_ascii_case("purple") {
            return Ok(PURPLE);
        }

        Err(ParseRgbError::new())
    }
}

impl Display for Rgb {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Rgb([r, g, b]) = *self;
        write!(f, "#{r:02X}{g:02X}{b:02X}")
    }
}

impl Rgb {
    #[inline]
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    #[inline]
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.0[2]
    }
}

pub fn blend(c1: Rgb, c2: Rgb, mid: f64) -> Rgb {
    let Rgb([r1, g1, b1]) = c1;
    let Rgb([r2, g2, b2]) = c2;

    let inv_mid = 1.0 - mid;
    let r = (r1 as f64 * inv_mid + r2 as f64 * mid).round();
    let g = (g1 as f64 * inv_mid + g2 as f64 * mid).round();
    let b = (b1 as f64 * inv_mid + b2 as f64 * mid).round();

    Rgb([r as u8, g as u8, b as u8])
}

