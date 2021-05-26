use std::collections::VecDeque;

use anyhow::Result;

use crate::data::asset::{BarDate, OHLCBar};

use super::{data_feed::DataFeed, LatestFeed, SourceFeed, TruncateFeed, UpdateFeed};

pub struct BarFeed(DataFeed<OHLCBar>);

impl BarFeed {
    pub fn new<V: Into<VecDeque<OHLCBar>>>(source: V) -> Self {
        Self(DataFeed::new(source))
    }
}

impl UpdateFeed for BarFeed {
    fn update(&mut self) -> bool {
        self.0.update()
    }
}

impl LatestFeed<OHLCBar> for BarFeed {
    fn latest(&self) -> Option<&OHLCBar> {
        self.0.latest()
    }

    fn latest_n(&self, n: usize) -> Box<dyn Iterator<Item = &OHLCBar> + '_> {
        self.0.latest_n(n)
    }
}

impl SourceFeed<OHLCBar> for BarFeed {
    fn source(&self) -> Option<&[OHLCBar]> {
        self.0.source()
    }
}

impl TruncateFeed for BarFeed {
    fn truncate(&mut self, date: BarDate) -> Result<()> {
        self.0.truncate(date)
    }
}
