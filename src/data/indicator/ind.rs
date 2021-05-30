use std::fmt::Debug;

use anyhow::Result;

use crate::data::{
    asset::{BarDate, GetBarDate, OHLCBar, Symbol},
    feed::{indicator_feed::IndicatorFeed, LatestFeed, SourceFeed, TruncateFeed, UpdateFeed},
    DataError,
};

use super::Indicator;

pub struct Ind<P, B: GetBarDate> {
    symbol: Symbol,
    params: P,
    bars: Option<IndicatorFeed<B>>,
    filler: Option<Box<dyn Fn(&P, &[OHLCBar]) -> Result<Vec<B>>>>,
}

impl<P, B: GetBarDate> Ind<P, B> {
    pub fn new<F>(symbol: &Symbol, params: P, filler: F) -> Self
    where
        F: Fn(&P, &[OHLCBar]) -> Result<Vec<B>> + 'static,
    {
        Self {
            symbol: symbol.clone(),
            bars: None,
            filler: Some(Box::new(filler)),
            params,
        }
    }
}

impl<P: Debug, B: Debug + GetBarDate> Indicator for Ind<P, B> {
    fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    fn fill(&mut self, bars: &[OHLCBar]) -> Result<BarDate> {
        let filler = self.filler.take().ok_or(DataError::InvalidIndicator)?;
        let bars = filler(&self.params, bars)?;
        let start_date = bars
            .first()
            .map(|bar| bar.bardate())
            .ok_or(DataError::NotEnoughSource)?;

        self.bars = Some(IndicatorFeed::new(bars));
        Ok(start_date)
    }
}

impl<P, B: GetBarDate> UpdateFeed for Ind<P, B> {
    fn update(&mut self) -> bool {
        if let Some(bars) = &mut self.bars {
            bars.update()
        } else {
            false
        }
    }
}

impl<P, B: GetBarDate> LatestFeed<B> for Ind<P, B> {
    fn latest(&self) -> Option<&B> {
        self.bars.as_ref().and_then(|feed| feed.latest())
    }

    fn latest_n(&self, n: usize) -> Box<dyn Iterator<Item = &B> + '_> {
        if let Some(feed) = &self.bars {
            feed.latest_n(n)
        } else {
            Box::new(std::iter::empty())
        }
    }
}

impl<P, B: GetBarDate> SourceFeed<B> for Ind<P, B> {
    fn source(&self) -> Option<&[B]> {
        self.bars.as_ref().and_then(|feed| feed.source())
    }
}

impl<P, B: GetBarDate> TruncateFeed for Ind<P, B> {
    fn truncate(&mut self, date: BarDate) -> Result<()> {
        self.bars
            .as_mut()
            .ok_or(DataError::InvalidIndicator.into())
            .and_then(|feed| feed.truncate(date))
    }
}
