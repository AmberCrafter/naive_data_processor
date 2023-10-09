use std::error::Error;

pub mod qc_module;
pub mod config_parser;
pub mod data_parser;
pub mod qc_worker;

type ERROR = Box<dyn Error + 'static>;