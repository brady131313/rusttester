use std::path::Path;

use anyhow::Result;

use crate::data::{asset::Symbol, data_handler::DataHandler, data_loader::csv_loader::CsvLoader};

pub mod data;
pub mod strategy;

pub fn run() -> Result<()> {
    let loader = CsvLoader::new(Path::new("./samples/"));
    let symbols = vec![
        Symbol::from("IVV"),
        Symbol::from("EFA"),
        Symbol::from("TLT"),
    ];

    let mut handler = DataHandler::new(&symbols, &loader)?;
    while handler.update() {
        if let Some(bar) = handler.ohlc(&symbols[0]) {
            print!("{:?}", bar.bardate);
        }
    }

    for symbol in &symbols {
        if let Some(data) = handler.ohlc_n(symbol, 9999) {
            println!("{} {:?}", symbol, data.count())
        }
    }

    Ok(())
}
