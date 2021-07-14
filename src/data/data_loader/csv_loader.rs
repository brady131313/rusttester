use std::{
    convert::TryFrom,
    num::{ParseFloatError, ParseIntError},
    path::Path,
};

use anyhow::Result;
use chrono::{NaiveDate, ParseError};
use csv::StringRecord;
use thiserror::Error;

use crate::data::asset::{BarDate, OHLCBar, Symbol};

use super::DataLoader;

#[derive(Error, Debug)]
pub enum CsvLoaderError {
    #[error("Invalid date")]
    ParseDateError(#[from] ParseError),
    #[error("Invalid float")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Invalid int")]
    ParseIntError(#[from] ParseIntError),
}

impl TryFrom<StringRecord> for OHLCBar {
    type Error = CsvLoaderError;

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        let bardate = BarDate::from(NaiveDate::parse_from_str(&value[0], "%F")?);
        let open = value[1].parse::<f64>()?;
        let high = value[2].parse::<f64>()?;
        let low = value[3].parse::<f64>()?;
        let close = value[4].parse::<f64>()?;
        let adj_close = value[5].parse::<f64>()?;
        let volume = value[6].parse::<u32>()?;

        Ok(Self {
            bardate,
            open,
            low,
            high,
            close,
            adj_close,
            volume,
        })
    }
}

pub struct CsvLoader<'a> {
    csv_dir: &'a Path,
}

impl<'a> CsvLoader<'a> {
    pub fn new(csv_dir: &'a Path) -> Self {
        Self { csv_dir }
    }
}

impl<'a> DataLoader for CsvLoader<'a> {
    fn load_symbol(&self, symbol: &Symbol) -> Result<Vec<OHLCBar>> {
        let path = self.csv_dir.join(symbol.to_string()).with_extension("csv");
        let rdr = csv::Reader::from_path(path)?;

        let data = rdr
            .into_records()
            .map(|r| OHLCBar::try_from(r.unwrap()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_loader() -> Result<()> {
        let loader = CsvLoader::new(Path::new("./samples/"));
        let data = loader.load_symbol(&Symbol::new("TEST")?)?;
        assert_eq!(data.len(), 4);
        Ok(())
    }
}
