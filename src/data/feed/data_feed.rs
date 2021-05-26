use std::collections::VecDeque;

use anyhow::Result;

use crate::data::{
    asset::{BarDate, GetBarDate},
    DataError,
};

use super::{LatestFeed, SourceFeed, TruncateFeed, UpdateFeed};

#[derive(Debug)]
pub struct DataFeed<T> {
    source: VecDeque<T>,
    latest: Vec<T>,
}

impl<T> DataFeed<T> {
    pub fn new<V: Into<VecDeque<T>>>(source: V) -> Self {
        Self {
            source: source.into(),
            latest: Vec::new(),
        }
    }
}

impl<T> UpdateFeed for DataFeed<T> {
    fn update(&mut self) -> bool {
        match self.source.pop_front() {
            Some(bar) => {
                self.latest.push(bar);
                true
            }
            None => false,
        }
    }
}

impl<T> LatestFeed<T> for DataFeed<T> {
    fn latest(&self) -> Option<&T> {
        self.latest.last()
    }

    fn latest_n(&self, n: usize) -> Box<dyn Iterator<Item = &T> + '_> {
        let iter = self.latest.iter().rev().take(n);
        Box::new(iter)
    }
}

impl<T> SourceFeed<T> for DataFeed<T> {
    fn source(&self) -> Option<&[T]> {
        if self.latest.is_empty() {
            let (front, _) = self.source.as_slices();
            Some(front)
        } else {
            None
        }
    }
}

impl<T: GetBarDate> TruncateFeed for DataFeed<T> {
    fn truncate(&mut self, date: BarDate) -> Result<()> {
        if !self.latest.is_empty() {
            return Err(DataError::InvalidIndicator.into());
        }

        self.source.retain(|item| date <= item.bardate());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::data::asset::OHLCBar;

    use super::*;

    #[test]
    fn test_latest() {
        let mut feed = DataFeed::new(vec![1, 2]);

        assert!(feed.latest().is_none());

        assert!(feed.update());
        assert_eq!(feed.latest().unwrap(), &1);

        assert!(feed.update());
        assert_eq!(feed.latest().unwrap(), &2);

        assert!(!feed.update());
        assert_eq!(feed.latest().unwrap(), &2);
    }

    #[test]
    fn test_latest_n() {
        let mut feed = DataFeed::new(vec![1, 2]);

        assert!(feed.latest_n(1).last().is_none());

        assert!(feed.update());
        assert_eq!(feed.latest_n(1).collect::<Vec<_>>(), vec![&1]);
        assert_eq!(feed.latest_n(5).collect::<Vec<_>>(), vec![&1]);

        assert!(feed.update());
        assert_eq!(feed.latest_n(1).collect::<Vec<_>>(), vec![&2]);
        assert_eq!(feed.latest_n(5).collect::<Vec<_>>(), vec![&2, &1]);

        assert!(!feed.update());
        assert_eq!(feed.latest_n(1).collect::<Vec<_>>(), vec![&2]);
        assert_eq!(feed.latest_n(5).collect::<Vec<_>>(), vec![&2, &1]);
    }

    #[test]
    fn test_source() {
        let mut feed = DataFeed::new(vec![1, 2]);

        assert_eq!(feed.source().unwrap(), vec![1, 2]);
        assert!(feed.update());
        assert!(feed.source().is_none());
    }

    #[test]
    fn test_truncate() {
        let mut feed = DataFeed::new(vec![
            OHLCBar {
                bardate: BarDate::from(NaiveDate::from_ymd(2021, 01, 01)),
                ..OHLCBar::default()
            },
            OHLCBar {
                bardate: BarDate::from(NaiveDate::from_ymd(2021, 01, 02)),
                ..OHLCBar::default()
            },
            OHLCBar {
                bardate: BarDate::from(NaiveDate::from_ymd(2021, 01, 03)),
                ..OHLCBar::default()
            },
        ]);

        let cutoff = BarDate::from(NaiveDate::from_ymd(2021, 01, 02));

        assert!(feed.truncate(cutoff).is_ok());

        assert!(feed.update());
        assert!(feed.update());
        assert!(!feed.update());

        assert_eq!(feed.latest_n(5).count(), 2);
    }
}
