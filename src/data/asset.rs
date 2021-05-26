use std::fmt;

use anyhow::Result;
use chrono::{NaiveDate, Utc};

use super::DataError;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Symbol(String);

impl Symbol {
    pub fn new(symbol: &str) -> Result<Self> {
        if symbol.is_empty() || symbol.len() > 5 {
            return Err(DataError::InvalidSymbol.into());
        }

        Ok(Self(String::from(symbol).to_uppercase()))
    }
}

impl From<&'static str> for Symbol {
    fn from(symbol: &'static str) -> Self {
        Self(symbol.to_owned())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct BarDate(NaiveDate);

impl Default for BarDate {
    fn default() -> Self {
        Self(Utc::today().naive_local())
    }
}

impl From<NaiveDate> for BarDate {
    fn from(date: NaiveDate) -> Self {
        Self(date)
    }
}

pub trait GetBarDate {
    fn bardate(&self) -> BarDate;
}

impl GetBarDate for OHLCBar {
    fn bardate(&self) -> BarDate {
        self.bardate
    }
}

#[derive(Debug, Default, Clone)]
pub struct OHLCBar {
    pub bardate: BarDate,
    pub open: f32,
    pub low: f32,
    pub high: f32,
    pub close: f32,
    pub adj_close: f32,
    pub volume: u32,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_symbol() -> Result<()> {
        let symbol = Symbol::new("IVV")?;
        assert_eq!(symbol.0, "IVV");

        let invalid = Symbol::new("123456");
        assert!(invalid.is_err());
        Ok(())
    }
}
