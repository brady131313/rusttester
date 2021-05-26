use anyhow::Result;

use super::asset::BarDate;

pub mod bar_feed;
mod data_feed;
pub mod indicator_feed;

pub trait UpdateFeed {
    fn update(&mut self) -> bool;
}

pub trait LatestFeed<T> {
    fn latest_n(&self, n: usize) -> Box<dyn Iterator<Item = &T> + '_>;

    fn latest(&self) -> Option<&T>;
}

pub trait SourceFeed<T> {
    fn source(&self) -> Option<&[T]>;
}

pub trait TruncateFeed {
    fn truncate(&mut self, date: BarDate) -> Result<()>;
}
