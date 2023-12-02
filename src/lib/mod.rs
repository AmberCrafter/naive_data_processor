use std::{error::Error, path::Path};

use chrono::NaiveDateTime;

use self::{general_module::GeneralModule, py_module::PythonModule, data_parser::DataType};

pub mod general_module;
pub mod config_parser;
pub mod data_parser;
pub mod qc_worker;
pub mod py_module;

type ERROR = Box<dyn Error + 'static>;

pub trait QCModule {
    fn run(&self, level: usize, datetime: &NaiveDateTime, data: &DataType) -> Result<bool, ERROR>;
}
