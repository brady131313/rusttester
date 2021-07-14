use anyhow::Result;

use super::{
    asset::{BarDate, OHLCBar, Symbol},
    feed::{TruncateFeed, UpdateFeed},
};

pub mod ind;

pub trait Indicator: UpdateFeed + TruncateFeed {
    fn symbol(&self) -> &Symbol;

    fn fill(&mut self, bars: &[OHLCBar]) -> Result<BarDate>;
}
