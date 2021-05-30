use anyhow::Result;

use super::{
    asset::{BarDate, OHLCBar, Symbol},
    feed::{TruncateFeed, UpdateFeed},
};

pub mod ema;
pub mod ind;
pub mod sma;

pub trait Indicator: UpdateFeed + TruncateFeed {
    fn symbol(&self) -> &Symbol;

    fn fill(&mut self, bars: &[OHLCBar]) -> Result<BarDate>;
}
