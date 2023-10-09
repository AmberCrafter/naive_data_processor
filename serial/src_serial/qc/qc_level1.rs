
use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::VecDeque;
use chrono::NaiveDateTime;

// #[macro_use]
// use crate::qc;

use crate::{condition_check, get_config};

use super::{metadata::Data, QCBoundaryCheck, QCConsistCheck, QCFlag, QCConditionCheck};

// boundary check
#[derive(Debug, Serialize, Deserialize)]
pub struct QCL1BC {
    pub temperature: Option<QCBoundaryCheck>,
    pub humidity: Option<QCBoundaryCheck>,
    pub pressure: Option<QCBoundaryCheck>,
    pub windspeed: Option<QCBoundaryCheck>,
    pub winddirection: Option<QCBoundaryCheck>,
    pub rainfull: Option<QCBoundaryCheck>,
    pub shortwavedown: Option<QCBoundaryCheck>,
    pub shortwaveup: Option<QCBoundaryCheck>,
    pub longwavedown: Option<QCBoundaryCheck>,
    pub longwaveup: Option<QCBoundaryCheck>,
}

// impl QCL1BC {
//     fn get(&self, field: &str) -> QCType {
//         match field {
//             "temperature" => QCType {
//                 BounaryCheck: self.temperature,
//             },
//             "humidty" => QCType {
//                 BounaryCheck: self.humidity,
//             },
//             "rainfull" => QCType {
//                 BounaryCheck: self.rainfull,
//             },
//             _ => QCType { Unknown: None },
//         }
//     }
// }

macro_rules! boundary_check {
    ($config: tt, $singal_data: tt, $all_pass: tt, $target: tt) => {
       if let Some(parameter) = $config.$target {
           let upper_bound = parameter.upper;
           let lower_bound = parameter.lower;
           if upper_bound.is_some() || lower_bound.is_some() {
                let length = $singal_data.$target.len();
                for idx in 0..length {
                    if let Some(limit) = upper_bound {
                        let cmp = $singal_data.$target[idx].partial_cmp(&limit);
                        if cmp == Some(Ordering::Greater) {
                            $singal_data.$target[idx] = f32::NAN;
                            $singal_data.flag |= QCFlag::L1;
                            $all_pass = false;
                        }
                    }
                }

                for idx in 0..length {
                    if let Some(limit) = lower_bound {
                        let cmp = $singal_data.$target[idx].partial_cmp(&limit);
                        if cmp == Some(Ordering::Less) {
                            $singal_data.$target[idx] = f32::NAN;
                            $singal_data.flag |= QCFlag::L1;
                            $all_pass = false;
                        }
                    }
                }
            }
        }
    };
}

fn boundary_check(config: QCL1BC, data: &mut Vec<Data>) -> bool {
    let mut all_pass = true;
    for ele in data {
        boundary_check!(config, ele, all_pass, temperature);
        boundary_check!(config, ele, all_pass, humidity);
        boundary_check!(config, ele, all_pass, pressure);
        boundary_check!(config, ele, all_pass, windspeed);
        boundary_check!(config, ele, all_pass, winddirection);
        boundary_check!(config, ele, all_pass, rainfull);
        boundary_check!(config, ele, all_pass, shortwavedown);
        boundary_check!(config, ele, all_pass, shortwaveup);
        boundary_check!(config, ele, all_pass, longwavedown);
        boundary_check!(config, ele, all_pass, longwaveup);
    }
    all_pass
}

// consistance check
#[derive(Debug, Serialize, Deserialize)]
pub struct QCL1CC {
    pub temperature: Option<QCConsistCheck>,
    pub humidity: Option<QCConsistCheck>,
    pub pressure: Option<QCConsistCheck>,
    pub windspeed: Option<QCConsistCheck>,
    pub winddirection: Option<QCConsistCheck>,
    pub rainfull: Option<QCConsistCheck>,
    pub shortwavedown: Option<QCConsistCheck>,
    pub shortwaveup: Option<QCConsistCheck>,
    pub longwavedown: Option<QCConsistCheck>,
    pub longwaveup: Option<QCConsistCheck>,
}

// impl QCL1_CC {
//     fn get(&self, field: &str) -> QCType {
//         match field {
//             "temperature" => QCType {
//                 ConsistCheck: self.temperature,
//             },
//             "humidty" => QCType {
//                 ConsistCheck: self.humidity,
//             },
//             _ => QCType { Unknown: None },
//         }
//     }
// }


macro_rules! consist_check {
    ($config: tt, $block_data: tt, $all_pass: tt, $target: tt) => {
        for column in 0..$block_data[0].$target.len() {
            if let Some(parameter) = $config.$target.as_ref() {
                if let Some(differs) = parameter.diff.as_ref() {
                    for differ in differs {
                        let (duration, limit) = (differ.duration as i64, differ.value);
                        let mut left = 0;
                        let mut cur;
                        while $block_data[left].$target[column]==f32::NAN {left+=1;}
                        cur = left;
                        let mut interval;
                        let mut diff_val;
                        let mut max_queue: VecDeque<(NaiveDateTime, _)> = VecDeque::new();  // (time, val)
                        let mut min_queue: VecDeque<(NaiveDateTime, _)> = VecDeque::new();  // (time, val)
                        let (mut cur_max, mut cur_min);
                        max_queue.push_back(($block_data[cur].datetime, $block_data[cur].$target[column]));
                        min_queue.push_back(($block_data[cur].datetime, $block_data[cur].$target[column]));
        
                        while cur<$block_data.len() {
                            interval = ($block_data[cur].datetime - $block_data[left].datetime).num_seconds();
                            if (($block_data[cur].$target[column] == f32::NAN) || (interval < duration)) {cur+=1; continue}

                            // max value
                            while max_queue.len()>1 && ($block_data[cur].datetime - max_queue[0].0).num_seconds() > duration {
                                max_queue.pop_front();
                            }
                            cur_max = max_queue[0].1;
                            if cur_max < $block_data[cur].$target[column] {
                                cur_max = $block_data[cur].$target[column];
                                max_queue.clear();
                                max_queue.push_back(($block_data[cur].datetime, cur_max));
                            } else {
                                while ($block_data[cur].datetime - max_queue[max_queue.len()-1].0).num_seconds() > duration ||  max_queue[max_queue.len()-1].1 < cur_max {
                                    max_queue.pop_back();
                                }
                                max_queue.push_back(($block_data[cur].datetime, $block_data[cur].$target[column]));
                            }

                            // min value
                            while min_queue.len()>1 && ($block_data[cur].datetime - min_queue[0].0).num_seconds() < duration {
                                min_queue.pop_front();
                            }
                            cur_min = min_queue[0].1;
                            if cur_min > $block_data[cur].$target[column] {
                                cur_min = $block_data[cur].$target[column];
                                min_queue.clear();
                                min_queue.push_back(($block_data[cur].datetime, cur_min));
                            } else {
                                while ($block_data[cur].datetime - min_queue[min_queue.len()-1].0).num_seconds() > duration ||  min_queue[min_queue.len()-1].1 < cur_min {
                                    min_queue.pop_back();
                                }
                                min_queue.push_back(($block_data[cur].datetime, $block_data[cur].$target[column]));
                            }

                            diff_val = ((cur_max - $block_data[cur].$target[column]).abs()).max(($block_data[cur].$target[column] - cur_min).abs());
                            if diff_val > (limit/duration as f32 * interval as f32) {
                                $block_data[cur].$target[column] = f32::NAN;
                                $block_data[cur].flag |= QCFlag::L1;
                                $all_pass = false;
                            }
                            left += 1;
                        }
                    }
                }
            }
        }
    }
}

fn _consist_check(config: QCL1CC, data: &mut Vec<Data>) -> bool {
    let mut all_pass = true;
    consist_check!(config, data, all_pass, temperature);
    consist_check!(config, data, all_pass, humidity);
    consist_check!(config, data, all_pass, pressure);
    consist_check!(config, data, all_pass, windspeed);
    consist_check!(config, data, all_pass, winddirection);
    consist_check!(config, data, all_pass, rainfull);
    consist_check!(config, data, all_pass, shortwavedown);
    consist_check!(config, data, all_pass, shortwaveup);
    consist_check!(config, data, all_pass, longwavedown);
    consist_check!(config, data, all_pass, longwaveup);
    all_pass
}

fn consist_check(config: QCL1CC, data: &mut Vec<Data>) -> bool {
    let mut all_pass = true;
    all_pass &= _consist_check(config, data);
    all_pass
}


// condition check
#[derive(Debug, Serialize, Deserialize)]
pub struct QCL1CD {
    pub temperature: Option<QCConditionCheck>,
    pub humidity: Option<QCConditionCheck>,
    pub pressure: Option<QCConditionCheck>,
    pub windspeed: Option<QCConditionCheck>,
    pub winddirection: Option<QCConditionCheck>,
    pub rainfull: Option<QCConditionCheck>,
    pub shortwavedown: Option<QCConditionCheck>,
    pub shortwaveup: Option<QCConditionCheck>,
    pub longwavedown: Option<QCConditionCheck>,
    pub longwaveup: Option<QCConditionCheck>,
}

fn valid_check(config: QCL1CD, data: &mut Vec<Data>) -> bool {
    let mut all_pass = true;
    for ele in data {
        condition_check!(config, ele, all_pass, QCFlag::L1, temperature, "valid");
        condition_check!(config, ele, all_pass, QCFlag::L1, humidity, "valid");
        condition_check!(config, ele, all_pass, QCFlag::L1, pressure, "valid");
        condition_check!(config, ele, all_pass, QCFlag::L1, windspeed, "valid");
        condition_check!(config, ele, all_pass, QCFlag::L1, winddirection, "valid");
        condition_check!(config, ele, all_pass, QCFlag::L1, rainfull, "valid");
        condition_check!(config, ele, all_pass, QCFlag::L1, shortwavedown, "valid");
        condition_check!(config, ele, all_pass, QCFlag::L1, shortwaveup, "valid");
        condition_check!(config, ele, all_pass, QCFlag::L1, longwavedown, "valid");
        condition_check!(config, ele, all_pass, QCFlag::L1, longwaveup, "valid");
    }
    all_pass
}


pub fn main(data: &mut Vec<Data>, config_path: &'static str) -> Result<bool, &'static str> {
    let mut all_pass = true;
    let config;
    get_config!(config, config_path, QCL1BC);
    all_pass &= boundary_check(config, data);

    let config;
    get_config!(config, config_path, QCL1CC);
    all_pass &= consist_check(config, data);

    let config;
    get_config!(config, config_path, QCL1CD);
    all_pass &= valid_check(config, data);

    Ok(all_pass)
}
