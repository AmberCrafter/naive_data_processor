use std::{collections::{HashMap, VecDeque}, fmt::Display};
use bitflags::bitflags;
use chrono::NaiveDateTime;

use crate::lib::config_parser::QCConfig;

use super::{data_parser::DataType, ERROR, qc_module::{qc_boundary, qc_consist}};

const MAX_BUFFER_SIZE: usize = 512;

#[derive(Debug, Clone, Copy)]
enum QCStatus {
    Unknown,
    Init,
    Training,
    Running,
    Stop,
}

bitflags! {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct QCFlag: u32 {
        const Clear = 0b0000_0000;
        const L0 = 0b0000_0001;
        const L1 = 0b0000_0010;
        const Invalid = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    }
}

impl QCFlag {
    pub fn new() -> Self {
        QCFlag::Clear
    }

    pub fn set_bit(&mut self, index: usize) {
        *self.0.bits_mut() |= 1<<index;
    }

    pub fn clear_all(&mut self){
        *self.0.bits_mut() = 0;
    }

    pub fn set_invalid(&mut self) {
        *self.0.bits_mut() = QCFlag::Invalid.bits();
    }
}

#[derive(Debug)]
struct WorkerStatus<T> {
    config: QCConfig,
    data: VecDeque<(NaiveDateTime, T)>,
    status: QCStatus,
    flag: QCFlag,
}

pub struct QCworker {
    map: HashMap<String, WorkerStatus<DataType>>,
}

impl WorkerStatus<DataType> {
    pub fn clean_flag(&mut self) {
        self.flag.clear_all();
    }

    pub fn qc_handle(&mut self, datetime: NaiveDateTime, data: DataType) {
        for level in 0..=self.config.max_level() {
            let level_pattern = self.config.members_mut(level);
            
            if let Some(conf) = &level_pattern.boundary {
                if !qc_boundary::main(conf, &data) {
                    self.data.push_back((datetime, DataType::NULL));
                    self.flag.set_bit(level);
                    return;
                }
            }

            if let Some(conf) = level_pattern.consist.as_mut() {
                if !qc_consist::main(conf, &datetime, &data) {
                    self.data.push_back((datetime, DataType::NULL));
                    self.flag.set_bit(level);
                    return;
                }
            }
        }
        self.data.push_back((datetime, data));
    }
}


impl QCworker {
    pub fn new<S: AsRef<str> + Display>(names: &[S]) -> Self {
        let mut map = HashMap::new();
        for name in names {
            if &name.to_string()[..] == "datetime" {continue;}
            map.insert(name.to_string(), WorkerStatus { 
                config: QCConfig::new(&format!("config/{}.toml", name)), 
                data: VecDeque::with_capacity(MAX_BUFFER_SIZE), 
                status: QCStatus::Init,
                flag: QCFlag::new(),
            });
        }
        QCworker { 
            map
        }
    }

    fn change_status(&mut self, target: String, status: QCStatus) -> Result<(), ERROR> {
        if let Some(val) = self.map.get_mut(&target) {
            val.status = status;
        }
        Ok(())
    }

    pub fn append<S: AsRef<str> + Display>(&mut self, target: S, dateitme: NaiveDateTime, data: DataType) {
        let mut entry = self.map.entry(target.to_string()).or_insert(
            WorkerStatus { 
                config: QCConfig::new(&format!("config/{}.toml", target)), 
                data: VecDeque::with_capacity(MAX_BUFFER_SIZE), 
                status: QCStatus::Init,
                flag: QCFlag::new(),
            }
        );
        entry.clean_flag();
        entry.qc_handle(dateitme, data);
    }

    pub fn show<S: AsRef<str> + Display>(&mut self, target: S) {
        if let Some(work) = self.map.get(&target.to_string()) {
            println!("{:#?}", work);
        } else {
            println!("Not exist: {:}", target);
        }
    }
}