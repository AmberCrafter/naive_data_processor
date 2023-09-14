use std::{fmt::Debug, path::PathBuf};

use super::QCFlag;
use crate::{get_config, utils::file::read_lines};
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

const HEADER_CONFIG_PATH: &str = "./config/header.toml";

#[derive(Debug, Clone, Copy)]
pub enum Parameter {
    Datetime,
    Temperature,
    Humidity,
    Pressure,
    Windspeed,
    WindDirection,
    Rainfall,
    ShortWaveDown,
    ShortWaveUp,
    LongWaveDown,
    LongWaveUp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderInner {
    header: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderVecInner {
    header: Option<Vec<String>>,
    primary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub datetime: Option<HeaderInner>,
    pub temperature: Option<HeaderVecInner>,
    pub humidity: Option<HeaderVecInner>,
    pub pressure: Option<HeaderVecInner>,
    pub windspeed: Option<HeaderVecInner>,
    pub winddirection: Option<HeaderVecInner>,
    pub rainfull: Option<HeaderVecInner>,
    pub shortwavedown: Option<HeaderVecInner>,
    pub shortwaveup: Option<HeaderVecInner>,
    pub longwavedown: Option<HeaderVecInner>,
    pub longwaveup: Option<HeaderVecInner>,
}

macro_rules! option_get_index {
    ($option: expr, $target: expr) => {
        if let Some(member) = $option {
            if let Some(arr) = member.header {
                arr.iter().position(|ele| ele == $target)
            } else {
                None
            }
        } else {
            None
        }
    };
}

impl Header {
    pub fn index(parameter: Parameter, header: &str) -> Option<usize> {
        let config;
        get_config!(config, HEADER_CONFIG_PATH, Header);

        match parameter {
            Parameter::Datetime => Some(0),
            Parameter::Temperature => option_get_index!(config.temperature, header),
            Parameter::Humidity => option_get_index!(config.humidity, header),
            Parameter::Pressure => option_get_index!(config.pressure, header),
            Parameter::Windspeed => option_get_index!(config.windspeed, header),
            Parameter::WindDirection => option_get_index!(config.winddirection, header),
            Parameter::Rainfall => option_get_index!(config.rainfull, header),
            Parameter::ShortWaveDown => option_get_index!(config.shortwavedown, header),
            Parameter::ShortWaveUp => option_get_index!(config.shortwaveup, header),
            Parameter::LongWaveDown => option_get_index!(config.longwavedown, header),
            Parameter::LongWaveUp => option_get_index!(config.longwaveup, header),
        }
    }

    pub fn find(header: &str) -> Option<(Parameter, usize)> {
        let config;
        get_config!(config, HEADER_CONFIG_PATH, Header);

        if let Some(idx) = Self::index(Parameter::Temperature, header) {
            return Some((Parameter::Temperature, idx));
        }
        if let Some(idx) = Self::index(Parameter::Humidity, header) {
            return Some((Parameter::Humidity, idx));
        }
        if let Some(idx) = Self::index(Parameter::Pressure, header) {
            return Some((Parameter::Pressure, idx));
        }
        if let Some(idx) = Self::index(Parameter::Windspeed, header) {
            return Some((Parameter::Windspeed, idx));
        }
        if let Some(idx) = Self::index(Parameter::WindDirection, header) {
            return Some((Parameter::WindDirection, idx));
        }
        if let Some(idx) = Self::index(Parameter::Rainfall, header) {
            return Some((Parameter::Rainfall, idx));
        }
        if let Some(idx) = Self::index(Parameter::ShortWaveDown, header) {
            return Some((Parameter::ShortWaveDown, idx));
        }
        if let Some(idx) = Self::index(Parameter::ShortWaveUp, header) {
            return Some((Parameter::ShortWaveUp, idx));
        }
        if let Some(idx) = Self::index(Parameter::LongWaveDown, header) {
            return Some((Parameter::LongWaveDown, idx));
        }
        if let Some(idx) = Self::index(Parameter::LongWaveUp, header) {
            return Some((Parameter::LongWaveUp, idx));
        }
        if let Some(idx) = Self::index(Parameter::Datetime, header) {
            return Some((Parameter::Datetime, idx));
        }
        None
    }
}

#[derive(Clone)]
pub struct Data {
    pub datetime: NaiveDateTime,
    pub temperature: Vec<f32>,
    pub humidity: Vec<f32>,
    pub pressure: Vec<f32>,
    pub windspeed: Vec<f32>,
    pub winddirection: Vec<f32>,
    pub rainfull: Vec<f32>,
    pub shortwavedown: Vec<f32>,
    pub shortwaveup: Vec<f32>,
    pub longwavedown: Vec<f32>,
    pub longwaveup: Vec<f32>,
    pub flag: QCFlag,
}

macro_rules! vec2str {
    ($data: expr) => {
        $data
            .iter()
            .map(|ele| ele.to_string())
            .collect::<Vec<String>>()
            .join(",")
    };
}

impl Data {
    pub fn to_vec(&self) -> Vec<String> {
        let mut res = Vec::new();
        res.push(self.datetime.to_string());
        res.push(vec2str!(self.temperature));
        res.push(vec2str!(self.humidity));
        res.push(vec2str!(self.pressure));
        res.push(vec2str!(self.windspeed));
        res.push(vec2str!(self.winddirection));
        res.push(vec2str!(self.rainfull));
        res.push(vec2str!(self.shortwavedown));
        res.push(vec2str!(self.shortwaveup));
        res.push(vec2str!(self.longwavedown));
        res.push(vec2str!(self.longwaveup));
        res.push(self.flag.bits().to_string());
        res
    }

    pub fn set_invalid(&mut self, parameter: Parameter) {
        match parameter {
            Parameter::Datetime => {}
            Parameter::Temperature => self.temperature.iter_mut().for_each(|e| *e = f32::NAN),
            Parameter::Humidity => self.humidity.iter_mut().for_each(|e| *e = f32::NAN),
            Parameter::Pressure => self.pressure.iter_mut().for_each(|e| *e = f32::NAN),
            Parameter::Windspeed => self.windspeed.iter_mut().for_each(|e| *e = f32::NAN),
            Parameter::WindDirection => self.winddirection.iter_mut().for_each(|e| *e = f32::NAN),
            Parameter::Rainfall => self.rainfull.iter_mut().for_each(|e| *e = f32::NAN),
            Parameter::ShortWaveDown => self.shortwavedown.iter_mut().for_each(|e| *e = f32::NAN),
            Parameter::ShortWaveUp => self.shortwaveup.iter_mut().for_each(|e| *e = f32::NAN),
            Parameter::LongWaveDown => self.longwavedown.iter_mut().for_each(|e| *e = f32::NAN),
            Parameter::LongWaveUp => self.longwaveup.iter_mut().for_each(|e| *e = f32::NAN),
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            datetime: NaiveDateTime::default(),
            temperature: Vec::new(),
            humidity: Vec::new(),
            pressure: Vec::new(),
            windspeed: Vec::new(),
            winddirection: Vec::new(),
            rainfull: Vec::new(),
            shortwavedown: Vec::new(),
            shortwaveup: Vec::new(),
            longwavedown: Vec::new(),
            longwaveup: Vec::new(),
            flag: QCFlag::default(),
        }
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "datetime: {:?}, temperature: {:?}, humidity: {:?}, pressure: {:?}, windspeed: {:?}, winddirection: {:?}, rainfull: {:?}, shortwavedown: {:?}, shortwaveup: {:?}, longwavedown: {:?}, longwaveup: {:?}", 
        self.datetime, self.temperature, self.humidity, self.pressure, self.windspeed, self.winddirection,
        self.rainfull, self.shortwavedown, self.shortwaveup, self.longwavedown, self.longwaveup)
    }
}

macro_rules! data_parser {
    ($dest: tt, $src: tt, datetime) => {
        $dest.datetime =
            NaiveDateTime::parse_from_str($src, "%Y-%m-%d %H:%M:%S").expect("Invalid time")
    };

    ($dest: tt, $src: tt, $type: ty, $target: tt, $default: expr) => {
        $dest.$target = $src.trim().parse::<$type>().unwrap_or($default)
    };

    // FIXME: need to push to specified position
    ($dest: tt, $src: tt, vector, $type: ty, $target: tt, $default: expr) => {
        $dest
            .$target
            .push($src.trim().parse::<$type>().unwrap_or($default))
    };
}

pub fn read_data(path: PathBuf) -> Vec<Data> {
    let mut res = Vec::new();
    if let Ok(reader) = read_lines(path) {
        let mut iter = reader.into_iter();
        let header = iter.next().expect("missing header").unwrap();
        let header = header.split(",").collect::<Vec<_>>();

        while let Some(line) = iter.next() {
            if let Ok(row) = line {
                let mut data = Data::default();
                for (val, key) in row.split(",").zip(header.clone()) {
                    // println!("{key:?}:{val:?}");

                    if let Some((class, _)) = Header::find(&key.trim().to_lowercase()[..]) {
                        // println!("{key:?}:{class:?}");
                        match class {
                            Parameter::Datetime => data_parser!(data, val, datetime),
                            Parameter::Temperature => {
                                data_parser!(data, val, vector, f32, temperature, f32::NAN)
                            },
                            Parameter::Humidity => {
                                data_parser!(data, val, vector, f32, humidity, f32::NAN)
                            },
                            Parameter::Pressure => {
                                data_parser!(data, val, vector, f32, pressure, f32::NAN)
                            },
                            Parameter::Windspeed => {
                                data_parser!(data, val, vector, f32, windspeed, f32::NAN)
                            },
                            Parameter::WindDirection => {
                                data_parser!(data, val, vector, f32, winddirection, f32::NAN)
                            },
                            Parameter::Rainfall => {
                                data_parser!(data, val, vector, f32, rainfull, f32::NAN)
                            },
                            Parameter::ShortWaveDown => {
                                data_parser!(data, val, vector, f32, shortwavedown, f32::NAN)
                            },
                            Parameter::ShortWaveUp => {
                                data_parser!(data, val, vector, f32, shortwaveup, f32::NAN)
                            },
                            Parameter::LongWaveDown => {
                                data_parser!(data, val, vector, f32, longwavedown, f32::NAN)
                            },
                            Parameter::LongWaveUp => {
                                data_parser!(data, val, vector, f32, longwaveup, f32::NAN)
                            },
                        }
                    } else {
                        println!(
                            "Error: {:?} not exist in header list",
                            &key.trim().to_lowercase()[..]
                        );
                    }
                }

                res.push(data);
            }
        }
    }
    res
}
