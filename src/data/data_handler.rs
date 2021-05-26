use std::collections::HashMap;

use anyhow::Result;
use chrono::NaiveDate;

use super::{
    asset::{BarDate, OHLCBar, Symbol},
    data_loader::DataLoader,
    feed::{bar_feed::BarFeed, LatestFeed, SourceFeed, TruncateFeed, UpdateFeed},
    indicator::Indicator,
    DataError,
};

pub struct DataHandler {
    ohlc_data: HashMap<Symbol, BarFeed>,
}

impl DataHandler {
    pub fn new(symbols: &[Symbol], loader: &impl DataLoader) -> Result<Self> {
        let ohlc_data = Self::prepare_data(symbols, loader)?;

        Ok(Self { ohlc_data })
    }

    pub fn ohlc(&self, symbol: &Symbol) -> Option<&OHLCBar> {
        self.ohlc_data.get(symbol).and_then(|feed| feed.latest())
    }

    pub fn ohlc_n(
        &self,
        symbol: &Symbol,
        n: usize,
    ) -> Option<Box<dyn Iterator<Item = &OHLCBar> + '_>> {
        self.ohlc_data.get(symbol).map(|feed| feed.latest_n(n))
    }

    pub fn update(&mut self) -> bool {
        for feed in self.ohlc_data.values_mut() {
            if !feed.update() {
                return false;
            }
        }

        true
    }

    pub fn fill_indicator(&self, indicator: &mut dyn Indicator) -> Result<BarDate> {
        let data = self
            .ohlc_data
            .get(indicator.symbol())
            .ok_or(DataError::InvalidSymbol)?;

        let bars = data.source().ok_or(DataError::SourceLocked)?;
        indicator.fill(bars)
    }

    fn prepare_data(
        symbols: &[Symbol],
        loader: &impl DataLoader,
    ) -> Result<HashMap<Symbol, BarFeed>> {
        let mut data = HashMap::with_capacity(symbols.len());
        let mut start_date = BarDate::from(NaiveDate::from_ymd(1900, 1, 1));

        for symbol in symbols {
            let bars = loader.load_symbol(&symbol)?;

            if let Some(bar) = bars.first() {
                if bar.bardate >= start_date {
                    start_date = bar.bardate
                }
            }

            let feed = BarFeed::new(bars);
            data.insert(symbol.clone(), feed);
        }

        for (_, feed) in &mut data {
            feed.truncate(start_date)?;
        }

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::data::data_loader::test_loader::TestLoader;

    use super::*;

    #[test]
    fn test_data_handler() -> Result<()> {
        let mut test_data = HashMap::new();
        test_data.insert(
            Symbol::from("IVV"),
            vec![
                OHLCBar {
                    bardate: BarDate::from(NaiveDate::from_ymd(2020, 01, 01)),
                    ..OHLCBar::default()
                },
                OHLCBar {
                    bardate: BarDate::from(NaiveDate::from_ymd(2020, 01, 02)),
                    ..OHLCBar::default()
                },
                OHLCBar {
                    bardate: BarDate::from(NaiveDate::from_ymd(2020, 01, 03)),
                    ..OHLCBar::default()
                },
            ],
        );

        test_data.insert(
            Symbol::from("EFA"),
            vec![
                OHLCBar {
                    bardate: BarDate::from(NaiveDate::from_ymd(2020, 01, 02)),
                    ..OHLCBar::default()
                },
                OHLCBar {
                    bardate: BarDate::from(NaiveDate::from_ymd(2020, 01, 03)),
                    ..OHLCBar::default()
                },
            ],
        );

        let loader = TestLoader::from(test_data);
        let symbols = vec![Symbol::from("IVV"), Symbol::from("EFA")];
        let mut handler = DataHandler::new(&symbols, &loader)?;

        // Handler only has two bars
        assert!(handler.update());
        assert!(handler.update());
        assert!(!handler.update());

        let bars = handler.ohlc_n(&symbols[0], 10).unwrap();
        assert_eq!(bars.count(), 2);

        Ok(())
    }
}
