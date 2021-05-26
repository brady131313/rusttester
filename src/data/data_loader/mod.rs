use anyhow::Result;

use super::asset::{OHLCBar, Symbol};

#[cfg(test)]
pub mod test_loader;

pub mod csv_loader;

pub trait DataLoader {
    fn load_symbol(&self, symbol: &Symbol) -> Result<Vec<OHLCBar>>;
}
