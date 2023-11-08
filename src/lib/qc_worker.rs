use bitflags::bitflags;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display, path::Path, net::{TcpListener, TcpStream}, error::Error, io::{BufReader, BufRead, Write, Read},
};
use toml::Table;

use crate::{lib::{config_parser::QCConfig, data_parser::data_parser_format}, get_config};

use super::{
    data_parser::{DataType, data_parser_key_value},
    qc_module::{qc_boundary, qc_consist},
    ERROR,
};

type BoxError = Box<dyn Error + 'static>;

const MAX_BUFFER_SIZE: usize = 512;
const ERROR_SHIFT: usize = 32;

#[derive(Debug, Clone, Copy)]
enum QCStatus {
    Unknown,
    Init,
    Training,
    Running,
    Stop,
}

// support 31 warning level, (0, 30)
// lower 32 bit as warning bit
// higher 32 bit as error bit, which set value as NAN
bitflags! {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct QCFlag: u64 {
        const Clear = 0b0000_0000;
        const L0_Warn = 0b0000_0001;
        const L1_Warn = 0b0000_0010;
        const L2_Warn = 0b0000_0100;
        const L3_Warn = 0b0000_1000;
        const L4_Warn = 0b0001_0000;
        const L5_Warn = 0b0010_0000;
        const L6_Warn = 0b0100_0000;
        const L7_Warn = 0b1000_0000;
        const Invalid = 0b1000_0000_0000_0000_0000_0000_0000_0000;
        const L0_Error = 1<<(ERROR_SHIFT + 0);
        const L1_Error = 1<<(ERROR_SHIFT + 1);
        const L2_Error = 1<<(ERROR_SHIFT + 2);
        const L3_Error = 1<<(ERROR_SHIFT + 3);
        const L4_Error = 1<<(ERROR_SHIFT + 4);
        const L5_Error = 1<<(ERROR_SHIFT + 5);
        const L6_Error = 1<<(ERROR_SHIFT + 6);
        const L7_Error = 1<<(ERROR_SHIFT + 7);
    }
}


impl QCFlag {
    pub fn new() -> Self {
        QCFlag::Clear
    }

    pub fn set_bit(&mut self, index: usize) {
        *self.0.bits_mut() |= 1 << index;
    }

    pub fn clear_all(&mut self) {
        *self.0.bits_mut() = 0;
    }

    pub fn set_invalid(&mut self) {
        *self.0.bits_mut() = QCFlag::Invalid.bits();
    }
}

#[derive(Debug)]
struct WorkerInner<T> {
    config: QCConfig,
    data: VecDeque<(NaiveDateTime, T)>,
    status: QCStatus,
    flag: QCFlag,
}

#[derive(Debug)]
pub struct QCworker {
    formation_table: HashMap<String, Vec<String>>,
    map: HashMap<String, WorkerInner<DataType>>,
}

impl WorkerInner<DataType> {
    pub fn new(parameter: &str) -> Self {
        WorkerInner {
            config: QCConfig::new(&format!("config/{}.toml", parameter)),
            data: VecDeque::with_capacity(MAX_BUFFER_SIZE),
            status: QCStatus::Init,
            flag: QCFlag::new(),
        }
    }

    pub fn new_with_size(parameter: &str, size: usize) -> Self {
        WorkerInner {
            config: QCConfig::new(&format!("config/{}.toml", parameter)),
            data: VecDeque::with_capacity(size),
            status: QCStatus::Init,
            flag: QCFlag::new(),
        }
    }

    pub fn clean_flag(&mut self) {
        self.flag.clear_all();
    }

    pub fn qc_handle(&mut self, datetime: NaiveDateTime, data: DataType) {
        for level in 0..=self.config.max_level() {
            let level_pattern = self.config.members_mut(level);

            if let Some(conf) = &level_pattern.boundary {
                if !qc_boundary::main(conf, &data) {
                    if let Some(errorflag) = &level_pattern.errorflag {
                        if *errorflag {
                            self.data.push_back((datetime, DataType::NULL));
                            self.flag.set_bit(level << ERROR_SHIFT);
                            return;
                        }
                    }
                    self.flag.set_bit(level);
                }
            }

            if let Some(conf) = level_pattern.consist.as_mut() {
                if !qc_consist::main(conf, &datetime, &data) {
                    if let Some(errorflag) = &level_pattern.errorflag {
                        if *errorflag {
                            self.data.push_back((datetime, DataType::NULL));
                            self.flag.set_bit(level << ERROR_SHIFT);
                            return;
                        }
                    }
                    self.flag.set_bit(level);
                }
            }

            // module
            if let Some(module_list) = level_pattern.module.as_mut() {}

        }
        self.data.push_back((datetime, data));
    }
}

impl QCworker {
    pub fn new(formation_table: HashMap<String, Vec<String>>) -> Self {
        let map = HashMap::new();
        QCworker { formation_table, map }
    }

    fn change_status(&mut self, target: String, status: QCStatus) -> Result<(), ERROR> {
        if let Some(val) = self.map.get_mut(&target) {
            val.status = status;
        }
        Ok(())
    }

    pub fn append<S: AsRef<str> + Display>(
        &mut self,
        target: S,
        datetime: NaiveDateTime,
        data: DataType,
    ) {
        let mut entry = self.map.entry(target.to_string())
            .or_insert(WorkerInner::new_with_size(target.as_ref(), MAX_BUFFER_SIZE));
        entry.clean_flag();
        entry.qc_handle(datetime, data);
    }

    pub fn show<S: AsRef<str> + Display>(&mut self, target: S) {
        if let Some(work) = self.map.get(&target.to_string()) {
            println!("{:#?}", work);
        } else {
            println!("Not exist: {:}", target);
        }
    }

    fn data_parse(&mut self, raw_data: &str) -> Option<Vec<(String, DataType)>> {
        if raw_data.starts_with("F") {
            let (protocol, payload) = raw_data.split_at(raw_data.find(",").unwrap());
            let payload = payload.strip_prefix(",").unwrap();
            // println!("protocol: {:?}, payload: {:?}", protocol, payload);

            let formation = self.formation_table.entry(protocol.to_string())
                .or_insert({
                    let path = "./config/formation_table.toml";
                    let cfg;
                    get_config!(cfg, path, FormationTable);
                    if let Some(v) = get_formations_table(&cfg, protocol) {
                        v
                    } else {
                        println!("TODO: return invalid information");
                        return None;
                    }
                });
            Some(data_parser_format(formation, payload))
        } else {
            Some(data_parser_key_value(raw_data))
        }
    }

    pub fn run(&mut self) {
        let mut buffer = String::new();
        loop {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();
    
            match buffer.as_str().trim() {
                "exit" | "q" | "quit" => {
                    break;
                },
                "" => {},
                s => {self.handler(s);}
            }
        }
    }

    pub fn handler(&mut self, raw_data: &str) {
        let current_datetime = chrono::offset::Local::now().naive_local();
        if let Some(arr) = self.data_parse(raw_data) {
            let datetime = if let Some(&(_, DataType::Datetime(dt))) = arr
                .iter()
                .filter(|(_, v)| {
                    match v {
                        DataType::Datetime(_) => true,
                        _ => false
                    }
                }).next() 
            {
                dt
            } else {
                current_datetime
            };

            for (target, data) in arr {
                self.append(target, datetime, data);
            }
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormationTable {
    formations: Table, // F{n} = []
}

pub fn get_formations_table(conf: &FormationTable, target: &str) -> Option<Vec<String>> {
    let table = &conf.formations;
    if table.contains_key(target) {
        if let Some(raw) = table.get(target) {
            if let Some(value) = raw.as_array() {
                let buf = value
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect();
                return Some(buf);
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::lib::data_parser::{data_parser_key_value, data_parser_format};

    use super::*;
    #[test]
    fn case1() {
        let path = "./config/formation_table.toml";
        let cfg;
        get_config!(cfg, path, FormationTable);

        println!("{cfg:?}");
    }

    #[test]
    fn case2() {
        let mut qc = QCworker::new(HashMap::new());
        qc.run();

        qc.show("temperature");
        qc.show("humidity");
    }
}
