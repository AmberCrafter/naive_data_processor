use chrono::NaiveDateTime;

use crate::lib::{data_parser::DataType, config_parser::Consist};

fn qc(config: &mut Vec<Consist>, datetime: &NaiveDateTime, data: f64) -> bool {
    if config.is_empty() {return false;}

    let mut ret = true;
    for conf in config {
        // check upper and lower config
        if conf.upper.len()==0 {
            conf.upper.push_back((datetime.clone(), data));
        }
        if conf.lower.len()==0 {
            conf.lower.push_back((datetime.clone(), data));
        }
        
        // check data inbound
        let (mut utime, mut udata) = conf.upper.front().unwrap();
        let (mut ltime, mut ldata) = conf.lower.front().unwrap();
        
        // update upper and lower config
        while (*datetime - utime).num_seconds() > conf.interval_to_sec() as i64 {
            conf.upper.pop_front();
            if conf.upper.len() > 0 {
                utime = conf.upper[0].0;
                udata = conf.upper[0].1;
            } else {
                break;
            }
        }

        while (*datetime - ltime).num_seconds() > conf.interval_to_sec() as i64 {
            conf.lower.pop_front();
            if conf.lower.len() > 0 {
                ltime = conf.lower[0].0;
                ldata = conf.lower[0].1;
            } else {
                break;
            }
        }
        
        if data>(ldata+conf.config.difference) || data<(udata-conf.config.difference) {
            ret = false;
            break;
        }

        // update upper and lower bound
        while data > udata {
            conf.upper.pop_front();
            if conf.upper.len() > 0 {
                // utime = conf.upper[0].0;
                udata = conf.upper[0].1;
            } else {
                break;
            }
        }

        while data < ldata {
            conf.lower.pop_front();
            if conf.lower.len() > 0 {
                // ltime = conf.lower[0].0;
                ldata = conf.lower[0].1;
            } else {
                break;
            }
        }

        if utime!=*datetime {
            conf.upper.push_back((datetime.clone(), data));
        }
        if ltime!=*datetime {
            conf.lower.push_back((datetime.clone(), data));
        }
    }
    ret
}

pub fn main(config: &mut Vec<Consist>, datetime: &NaiveDateTime,data: &DataType) -> bool {
    match data {
        DataType::Float(val) => {
            qc(config, datetime, *val)
        }
        _ => false        
    }
}