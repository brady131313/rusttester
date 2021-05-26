use std::collections::HashMap;

use anyhow::Result;

use crate::data::asset::{OHLCBar, Symbol};

use super::DataLoader;

pub struct TestLoader {
    data: HashMap<Symbol, Vec<OHLCBar>>,
}

impl TestLoader {
    pub fn new() -> Self {
        let data = HashMap::new();
        Self { data }
    }
}

impl From<HashMap<Symbol, Vec<OHLCBar>>> for TestLoader {
    fn from(data: HashMap<Symbol, Vec<OHLCBar>>) -> Self {
        Self { data }
    }
}

impl DataLoader for TestLoader {
    fn load_symbol(&self, symbol: &Symbol) -> Result<Vec<OHLCBar>> {
        if self.data.is_empty() {
            let data = vec![OHLCBar::default(), OHLCBar::default(), OHLCBar::default()];
            Ok(data)
        } else {
            let data = self
                .data
                .get(symbol)
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<_>>();

            Ok(data)
        }
    }
}
