use std::ops::{Deref, DerefMut};

use anyhow::Result;

use crate::data::{
    asset::{BarDate, GetBarDate, OHLCBar, Symbol},
    DataError,
};

use super::ind::Ind;

#[derive(Debug)]
pub struct SmaBar {
    pub bardate: BarDate,
    pub value: f32,
}

impl GetBarDate for SmaBar {
    fn bardate(&self) -> BarDate {
        self.bardate
    }
}

pub struct Sma(Ind<usize, SmaBar>);

impl Sma {
    pub fn new(symbol: &Symbol, period: usize) -> Self {
        Self(Ind::new(symbol, period, Self::calculate))
    }

    fn calculate(period: &usize, bars: &[OHLCBar]) -> Result<Vec<SmaBar>> {
        if bars.len() < *period {
            return Err(DataError::NotEnoughSource.into());
        }

        let mut window: Vec<f32> = Vec::with_capacity(*period);
        let mut ma: Vec<SmaBar> = Vec::with_capacity(bars.len() - period + 1);

        for bar in bars {
            window.push(bar.close);

            if window.len() == *period {
                let avg = window.iter().sum::<f32>() / *period as f32;
                ma.push(SmaBar {
                    bardate: bar.bardate,
                    value: avg,
                });

                window.remove(0);
            }
        }

        Ok(ma)
    }
}

impl Deref for Sma {
    type Target = Ind<usize, SmaBar>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sma {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chrono::NaiveDate;

    use crate::data::{
        data_loader::{test_loader::TestLoader, DataLoader},
        feed::SourceFeed,
        indicator::Indicator,
    };

    use super::*;

    #[test]
    fn test_simple_moving_average() -> Result<()> {
        let symbol = Symbol::from("IVV");
        let mut mock_data = HashMap::new();
        mock_data.insert(
            symbol.clone(),
            vec![
                OHLCBar {
                    bardate: BarDate::from(NaiveDate::from_ymd(2021, 5, 1)),
                    close: 1.0,
                    ..OHLCBar::default()
                },
                OHLCBar {
                    bardate: BarDate::from(NaiveDate::from_ymd(2021, 5, 2)),
                    close: 2.0,
                    ..OHLCBar::default()
                },
                OHLCBar {
                    bardate: BarDate::from(NaiveDate::from_ymd(2021, 5, 3)),
                    close: 3.0,
                    ..OHLCBar::default()
                },
                OHLCBar {
                    bardate: BarDate::from(NaiveDate::from_ymd(2021, 5, 4)),
                    close: 2.0,
                    ..OHLCBar::default()
                },
                OHLCBar {
                    bardate: BarDate::from(NaiveDate::from_ymd(2021, 5, 5)),
                    close: 1.0,
                    ..OHLCBar::default()
                },
            ],
        );

        let test_loader = TestLoader::from(mock_data);
        let bars = test_loader.load_symbol(&symbol)?;

        let mut sma = Sma::new(&symbol, 3);

        let start_date = sma.fill(&bars)?;
        assert_eq!(start_date, BarDate::from(NaiveDate::from_ymd(2021, 5, 3)));

        let mut output = sma.source().unwrap().iter();
        assert_abs_diff_eq!(output.next().unwrap().value, 2.0, epsilon = 0.0001);
        assert_abs_diff_eq!(output.next().unwrap().value, 2.3333, epsilon = 0.0001);
        assert_abs_diff_eq!(output.next().unwrap().value, 2.0, epsilon = 0.0001);

        Ok(())
    }
}
