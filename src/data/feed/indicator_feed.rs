use anyhow::Result;

use crate::data::asset::{BarDate, GetBarDate};

use super::{data_feed::DataFeed, LatestFeed, TruncateFeed, UpdateFeed};

pub struct IndicatorFeed<T>(DataFeed<T>);

impl<T> UpdateFeed for IndicatorFeed<T> {
    fn update(&mut self) -> bool {
        self.0.update()
    }
}

impl<T> LatestFeed<T> for IndicatorFeed<T> {
    fn latest(&self) -> Option<&T> {
        self.0.latest()
    }

    fn latest_n(&self, n: usize) -> Box<dyn Iterator<Item = &T> + '_> {
        self.0.latest_n(n)
    }
}

impl<T: GetBarDate> TruncateFeed for IndicatorFeed<T> {
    fn truncate(&mut self, date: BarDate) -> Result<()> {
        self.0.truncate(date)
    }
}
