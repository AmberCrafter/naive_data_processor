use std::{path::Path, collections::VecDeque};

use chrono::NaiveDateTime;
use serde_derive::{Serialize, Deserialize};
use toml::Table;

#[macro_export]
macro_rules! get_config {
    ($config: expr, $path: path, $ret_type: ty) => {
        let mut file = std::fs::File::open($path)
            .expect("Can't open config file.");
        let mut contents = String::new();
        std::io::Read::read_to_string(&mut file, &mut contents)
            .expect("Failed to read file.");

        $config = toml::from_str::<$ret_type>(&contents).expect("Parsing config failed");
    };
}

#[derive(Debug, Serialize, Deserialize)]
struct Meatadata {
    max_level: u64
}

#[derive(Debug, Default)]
pub struct LevelPattern {
    pub boundary: Option<Boundary>,
    pub consist: Option<Vec<Consist>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Boundary {
    pub max: f64,
    pub min: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsistConfig {
    pub interval: u64,
    pub unit: String,
    pub difference: f64
}

#[derive(Debug)]
pub struct Consist {
    pub config: ConsistConfig,
    pub upper: VecDeque<(NaiveDateTime, f64)>,
    pub lower: VecDeque<(NaiveDateTime, f64)>,
}

#[derive(Debug)]
pub struct QCConfig {
    metadata: Meatadata,
    levels: Vec<LevelPattern>
}

impl Consist {
    pub fn interval_to_sec(&self) -> u64 {
        let multi = match &self.config.unit.to_lowercase()[..] {
            "m" | "min" | "minute" => 60,
            "h" | "hr"  | "hour" => 60 * 60,
            "d" | "day" => 24 * 60 * 60,
            _ => 1
        };

        self.config.interval * multi
    }
}

impl QCConfig {
    pub fn new(path: &str) -> Self {
        let data; 
        get_config!(data, path, Table);

        // println!("{data:?}");

        let metadata = Meatadata { max_level: data["Global"]["max_level"].as_integer().unwrap() as u64};
        let mut levels = Vec::new();

        for i in 0..=metadata.max_level {
            let section = format!("level_{}", i);
            if let Some(table) = data[&section].as_table() {
                levels.push(QCConfig::parse_level(table));
            }
        }

        Self {
            metadata,
            levels,
        }
    }

    fn parse_level(data: &Table) -> LevelPattern {
        let mut res = LevelPattern::default();

        if data.contains_key("boundary") {
            if let Some(value) = data["boundary"].as_table() {
                res.boundary = Some(Boundary { 
                    max: value["max"].as_float().unwrap(), 
                    min: value["min"].as_float().unwrap(), 
                })
            }
        }

        if data.contains_key("consist") {
            if let Some(values) = data["consist"].as_array() {
                let mut buffer = Vec::new();
                for value in values {
                    if let Some(value) = value.as_table() {
                        buffer.push(Consist {
                            config: ConsistConfig { 
                                interval: value["interval"].as_integer().unwrap() as u64, 
                                unit: value["unit"].as_str().unwrap().to_string(), 
                                difference: value["difference"].as_float().unwrap(), 
                            },
                            upper: VecDeque::new(),
                            lower: VecDeque::new(),
                        })
                    }
                }
                res.consist = Some(buffer);
            }
        }
        res
    }

    pub fn max_level(&self) -> usize {
        self.metadata.max_level as usize
    }

    pub fn members(&self, level: usize) -> &LevelPattern {
        &self.levels[level]
    }

    pub fn members_mut(&mut self, level: usize) -> &mut LevelPattern {
        &mut self.levels[level]
    }
}


#[cfg(test)]
mod tests {
    use super::QCConfig;

    #[test]
    fn testcase1() {
        let qc = QCConfig::new("./config/temperature.toml");
        println!("{qc:?}");
    }
}