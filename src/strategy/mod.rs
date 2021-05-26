use std::any::type_name;

use anyhow::Result;

use crate::data::{asset::BarDate, data_handler::DataHandler, indicator::Indicator, DataError};

pub trait Strategy {
    fn indicators_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut dyn Indicator> + 'a> {
        Box::new(std::iter::empty())
    }

    fn fill_indicators(&mut self, handler: &DataHandler) -> Result<BarDate> {
        self.indicators_mut()
            .map(|ind| handler.fill_indicator(ind))
            .max_by_key(|res| match res {
                Ok(date) => date.to_owned(),
                _ => BarDate::default(),
            })
            .ok_or(DataError::InvalidIndicator)?
    }

    fn update_indicators(&mut self) -> bool {
        self.indicators_mut()
            .map(|ind| ind.update())
            .all(|updated| updated)
    }

    fn truncate_indicators(&mut self, date: BarDate) -> Result<()> {
        self.indicators_mut()
            .map(|ind| ind.truncate(date))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        type_name::<Self>().split("::").last().unwrap()
    }
}
