use crate::lib::{data_parser::DataType, config_parser::Boundary};

fn qc(config: &Boundary, data: f64) -> bool {
    if data >= config.min && data <= config.max {
        return true
    }
    return false;
}

pub fn main(config: &Boundary, data: &DataType) -> bool {
    match data {
        DataType::Float(val) => {
            qc(config, *val)
        },
        DataType::Integer(val) => {
            qc(config, *val as f64)
        },
        _ => false        
    }
}