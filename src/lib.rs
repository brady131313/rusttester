use std::path::Path;

use anyhow::Result;
use rustta::indicators::overlap_studies::SmaBuilder;

use crate::data::{
    asset::Symbol,
    data_handler::DataHandler,
    data_loader::{csv_loader::CsvLoader, DataLoader},
};

pub mod data;
pub mod strategy;

pub fn run() -> Result<()> {
    let loader = CsvLoader::new(Path::new("./samples/"));
    let symbols = vec![
        Symbol::from("IVV"),
        Symbol::from("EFA"),
        Symbol::from("TLT"),
    ];

    let sma = SmaBuilder::default().time_period(30).build()?;

    let data = loader.load_symbol(&Symbol::from("IVV"))?;
    let close = data.iter().map(|bar| bar.open).collect::<Vec<_>>();
    let output = sma.calculate(close);
    dbg!(output);
    /*
    let mut handler = DataHandler::new(&symbols, &loader)?;
    while handler.update() {
        if let Some(bar) = handler.ohlc(&symbols[0]) {
            print!("{:?}", bar.bardate);
        }
    }
    */

    Ok(())
}
