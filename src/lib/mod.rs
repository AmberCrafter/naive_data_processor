use std::error::Error;

use chrono::NaiveDateTime;

use self::data_parser::DataType;

pub mod config_parser;
pub mod data_parser;
pub mod general_module;
pub mod py_module;
pub mod qc_worker;
pub mod database;

type ERROR = Box<dyn Error + 'static>;

pub trait QCModule {
    fn run(&self, level: usize, datetime: &NaiveDateTime, data: &DataType) -> Result<bool, ERROR>;
}
