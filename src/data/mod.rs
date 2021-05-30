use thiserror::Error;

pub mod asset;
pub mod data_handler;
pub mod data_loader;
pub mod feed;
pub mod indicator;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("symbol must be shorter than 5 chars")]
    InvalidSymbol,
    #[error("Not enough source data")]
    NotEnoughSource,
    #[error("cant access data source after backtest start")]
    SourceLocked,
    #[error("error filling indicator")]
    InvalidIndicator,
}
