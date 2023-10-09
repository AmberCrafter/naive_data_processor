use std::error::Error;

use chrono::NaiveDateTime;
use serde_derive::{Serialize, Deserialize};

use crate::{get_config, qc::{QCFlag, metadata::{Parameter, Header}}};

use super::metadata::Data;


#[derive(Debug, Serialize, Deserialize)]
pub struct BlackListInner {
    datetimelst: Option<Vec<Vec<String>>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhiteListInner {
    datetimelst: Option<Vec<Vec<String>>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QCL0WBList {
    pub whitelist: Option<WhiteListInner>,
    pub blacklist: Option<BlackListInner>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct _WBListInner {
    pub starttime: NaiveDateTime,
    pub endtime: NaiveDateTime,
    pub blacklist: bool,
    pub parameters: Vec<String>
}

#[derive(Debug)]
pub struct WBList {
    pub inner: Vec<_WBListInner>
}

impl WBList {
    pub fn new(config: QCL0WBList) -> Result<Self, Box<dyn Error + 'static>> {
        let mut inner = Vec::new();
        if let Some(wlist) = config.whitelist {
            if let Some(rules) = wlist.datetimelst {
                for rule in rules {
                    let st = NaiveDateTime::parse_from_str(&rule[0], "%Y-%m-%d %H:%M:%S")?;
                    let et = NaiveDateTime::parse_from_str(&rule[1], "%Y-%m-%d %H:%M:%S")?;
                    let paras = rule[2..].to_vec();
                    inner.push(
                        _WBListInner {
                            starttime: st,
                            endtime: et,
                            blacklist: false,
                            parameters: paras
                        }
                    )
                }
            }
        }

        if let Some(blist) = config.blacklist {
            if let Some(rules) = blist.datetimelst {
                for rule in rules {
                    let st = NaiveDateTime::parse_from_str(&rule[0], "%Y-%m-%d %H:%M:%S")?;
                    let et = NaiveDateTime::parse_from_str(&rule[1], "%Y-%m-%d %H:%M:%S")?;
                    let paras = rule[2..].to_vec();
                    inner.push(
                        _WBListInner {
                            starttime: st,
                            endtime: et,
                            blacklist: true,
                            parameters: paras
                        }
                    )
                }
            }
        }

        inner.sort();

        Ok(WBList { inner })
    }
}

pub fn main(data: &mut Vec<Data>, config_path: &'static str) -> Result<(), Box<dyn Error + 'static>> {
    
    let raw_config;
    get_config!(raw_config, config_path, QCL0WBList);
    println!("{:?}", raw_config);

    let config = WBList::new(raw_config)?;

    let mut cfg_index = 0;
    for idx in 0..data.len() {
        while cfg_index<config.inner.len() && data[idx].datetime > config.inner[cfg_index].endtime {
            cfg_index+=1;
        }
        if cfg_index>=config.inner.len() {
            break;
        }
        if data[idx].datetime < config.inner[cfg_index].starttime {
            continue;
        }
        if config.inner[cfg_index].blacklist {
            data[idx].flag |= QCFlag::Invalid;
            for para in &config.inner[cfg_index].parameters {
                if let Some((parameter, i)) = Header::find(para) {
                    match parameter {
                        Parameter::Datetime => {},
                        Parameter::Temperature => data[idx].temperature[i]=f32::NAN,
                        Parameter::Humidity => data[idx].humidity[i]=f32::NAN,
                        Parameter::Pressure => data[idx].pressure[i]=f32::NAN,
                        Parameter::Windspeed => data[idx].windspeed[i]=f32::NAN,
                        Parameter::WindDirection => data[idx].winddirection[i]=f32::NAN,
                        Parameter::Rainfall => data[idx].rainfull[i]=f32::NAN,
                        Parameter::ShortWaveDown => data[idx].shortwavedown[i]=f32::NAN,
                        Parameter::ShortWaveUp => data[idx].shortwaveup[i]=f32::NAN,
                        Parameter::LongWaveDown => data[idx].longwavedown[i]=f32::NAN,
                        Parameter::LongWaveUp => data[idx].longwaveup[i]=f32::NAN,
                    }
                } else {
                    match &para[..] {
                        "none" => {},
                        "all" => {
                            data[idx].set_invalid(Parameter::Temperature);
                            data[idx].set_invalid(Parameter::Humidity);
                            data[idx].set_invalid(Parameter::Pressure);
                            data[idx].set_invalid(Parameter::Windspeed);
                            data[idx].set_invalid(Parameter::WindDirection);
                            data[idx].set_invalid(Parameter::Rainfall);
                            data[idx].set_invalid(Parameter::ShortWaveDown);
                            data[idx].set_invalid(Parameter::ShortWaveUp);
                            data[idx].set_invalid(Parameter::LongWaveDown);
                            data[idx].set_invalid(Parameter::LongWaveUp);
                        }
                        others => {println!("Not support element: {:?}", others); },
                    }
                }
            }
        }
    }
    Ok(())
}

