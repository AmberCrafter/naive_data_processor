use std::{path::Path, collections::VecDeque, fmt::Debug};

use chrono::NaiveDateTime;
use serde_derive::{Serialize, Deserialize};
use toml::Table;

use super::QCModule;

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
    pub module: Option<Vec<ExtModule>>,
    pub errorflag: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModuleType {
    Unknown = 0,
    General = 1,
    Python = 2,
}

impl<T: AsRef<str>> From<T> for ModuleType {
    fn from(value: T) -> Self {
        match value.as_ref().to_lowercase().as_str() {
            "general" => ModuleType::General,
            "c" => ModuleType::General,
            "rust" => ModuleType::General,
            "python" => ModuleType::Python,
            _ => ModuleType::Unknown
        }
    }
}

#[derive(Debug)]
pub struct ExtModule {
    pub name: String,
    pub module_type: ModuleType,
    pub path: String,
    pub instance: Option<Box<dyn QCModule + 'static>>,
    pub errorflag: bool,
}

impl Debug for (dyn QCModule + 'static) {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("QCModule")
    }
}

#[derive(Debug)]
pub struct QCConfig {
    metadata: Meatadata,
    levels: Vec<LevelPattern>
}

impl QCConfig {
    pub fn new(path: &str) -> Self {
        let data; 
        // println!("{path:?}");

        get_config!(data, path, Table);

        // println!("{data:?}");

        let metadata = Meatadata { max_level: data["Global"]["max_level"].as_integer().unwrap() as u64};
        let mut levels = Vec::new();

        for i in 0..=metadata.max_level {
            let section = format!("level_{}", i);

            if !data.contains_key(&section) {
                // TODO: Record error
                continue;
            }

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

        if data.contains_key("module") {
            if let Some(values) = data["module"].as_array() {
                let mut module_list = Vec::new();
                for value in values {
                    if let Some(val) = value.as_table() {
                        module_list.push(ExtModule {
                            name: val["name"].as_str().unwrap().to_string(),
                            module_type: ModuleType::from(
                                val["module_type"].as_str().unwrap()
                            ), 
                            path: val["path"].as_str().unwrap().to_string(),
                            instance: None,
                            errorflag: if let Some(toml::Value::Boolean(v)) = val.get("errorflag") {
                                *v
                            } else {
                                false
                            },
                        });
                    }
                }
                res.module = Some(module_list);
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