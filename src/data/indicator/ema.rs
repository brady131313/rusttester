use std::ops::{Deref, DerefMut};

use anyhow::Result;

use crate::data::{
    asset::{BarDate, GetBarDate, OHLCBar, Symbol},
    DataError,
};

use super::ind::Ind;

#[derive(Debug)]
pub struct EmaBar {
    pub bardate: BarDate,
    pub value: f32,
}

impl GetBarDate for EmaBar {
    fn bardate(&self) -> BarDate {
        self.bardate
    }
}

pub struct Ema(Ind<(usize, usize), EmaBar>);

impl Ema {
    pub fn new(symbol: &Symbol, period: usize, smoothing: Option<usize>) -> Self {
        let smoothing = smoothing.unwrap_or(2);
        Self(Ind::new(symbol, (period, smoothing), Self::calculate))
    }

    fn calculate(params: &(usize, usize), bars: &[OHLCBar]) -> Result<Vec<EmaBar>> {
        if bars.len() < params.0 {
            return Err(DataError::NotEnoughSource.into());
        }

        let smoothing = params.1 / (1 + params.0);
        let mut window: Vec<f32> = Vec::with_capacity(params.0);
        let mut ma: Vec<EmaBar> = Vec::with_capacity(bars.len() - params.0 + 1);

        for bar in bars {
            window.push(bar.close);

            if window.len() == params.0 {
                let avg = 0.0;
                ma.push(EmaBar {
                    bardate: bar.bardate,
                    value: avg,
                });

                window.remove(0);
            }
        }

        Ok(ma)
    }
}

impl Deref for Ema {
    type Target = Ind<(usize, usize), EmaBar>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ema {
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
    fn test_exponential_moving_average() -> Result<()> {
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

        let mut ema = Ema::new(&symbol, 3, None);

        let start_date = ema.fill(&bars)?;
        assert_eq!(start_date, BarDate::from(NaiveDate::from_ymd(2021, 5, 3)));

        let mut output = ema.source().unwrap().iter();
        assert_abs_diff_eq!(output.next().unwrap().value, 2.0, epsilon = 0.0001);
        assert_abs_diff_eq!(output.next().unwrap().value, 2.0, epsilon = 0.0001);
        assert_abs_diff_eq!(output.next().unwrap().value, 2.0, epsilon = 0.0001);

        Ok(())
    }
}
