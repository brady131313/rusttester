use std::fmt::Debug;

use anyhow::Result;

use super::{
    asset::{BarDate, OHLCBar, Symbol},
    feed::{TruncateFeed, UpdateFeed},
};

pub trait Indicator: Debug + UpdateFeed + TruncateFeed {
    fn symbol(&self) -> &Symbol;

    fn fill(&mut self, bars: &[OHLCBar]) -> Result<BarDate>;
}
