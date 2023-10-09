pub mod qc_level0;
pub mod qc_level1;
pub mod metadata;

// use std::mem::ManuallyDrop;

use bitflags::bitflags;
use serde_derive::{Serialize, Deserialize};

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

bitflags! {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct QCFlag: u32 {
        const Clear = 0b0000_0000;
        const L1 = 0b0000_0001;
        const L2 = 0b0000_0010;
        const Invalid = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    }
}

impl QCFlag {
    pub fn new() -> Self {
        QCFlag::Clear
    }

    pub fn clear(&mut self){
        *self.0.bits_mut() = 0;
    }
}

// // #[derive(Debug, Clone, Copy)]
// pub union QCType {
//     Unknown: Option<i32>,
//     BounaryCheck: QCBoundaryCheck, 
//     ConsistCheck: ManuallyDrop<QCConsistCheck>,
// }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QCBoundaryCheck {
    pub upper: Option<f32>,
    pub lower: Option<f32>
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QCConsistCheckDiffInner {
    pub duration: usize,
    pub value: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCConsistCheck {
    pub diff: Option<Vec<QCConsistCheckDiffInner>>
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCConditionCheckDiffInner {
    pub valid: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QCConditionCheck {
    pub condition: Option<QCConditionCheckDiffInner>
}

#[macro_export]
macro_rules! condition_check {
    ($config: tt, $singal_data: tt, $all_pass: tt, $level: expr, $target: tt, $condition_member: literal) => {
        if let Some(parameter) = $config.$target.as_ref() {
            if let Some(condition) = parameter.condition.as_ref() {
                let is_valid = match $condition_member {
                    "valid" => {
                        let mut is_valid = true;
                        for ele in &condition.valid {
                            if let Some((parameter, idx)) = crate::qc::metadata::Header::find(ele) {
                                is_valid &= match parameter {
                                    crate::qc::metadata::Parameter::Datetime => true,
                                    crate::qc::metadata::Parameter::Temperature => !$singal_data.temperature[idx].is_nan(),
                                    crate::qc::metadata::Parameter::Humidity => !$singal_data.humidity[idx].is_nan(),
                                    crate::qc::metadata::Parameter::Pressure => !$singal_data.pressure[idx].is_nan(),
                                    crate::qc::metadata::Parameter::Windspeed => !$singal_data.windspeed[idx].is_nan(),
                                    crate::qc::metadata::Parameter::WindDirection => !$singal_data.winddirection[idx].is_nan(),
                                    crate::qc::metadata::Parameter::Rainfall => !$singal_data.rainfull[idx].is_nan(),
                                    crate::qc::metadata::Parameter::ShortWaveDown => !$singal_data.shortwavedown[idx].is_nan(),
                                    crate::qc::metadata::Parameter::ShortWaveUp => !$singal_data.shortwaveup[idx].is_nan(),
                                    crate::qc::metadata::Parameter::LongWaveDown => !$singal_data.longwavedown[idx].is_nan(),
                                    crate::qc::metadata::Parameter::LongWaveUp => !$singal_data.longwaveup[idx].is_nan(),
                                }
                            } else  {
                                println!("Not support element: {:?}", ele);
                                is_valid = false;
                            }
                        }
                        is_valid
                    },
                    "nonzero" => {
                        let mut is_valid = true;
                        for ele in &condition.valid {
                            if let Some((parameter, idx)) = crate::qc::metadata::Header::find(ele) {
                                is_valid &= match parameter {
                                    crate::qc::metadata::Parameter::Datetime => true,
                                    crate::qc::metadata::Parameter::Temperature => $singal_data.temperature[idx]!=0.0,
                                    crate::qc::metadata::Parameter::Humidity => $singal_data.humidity[idx]!=0.0,
                                    crate::qc::metadata::Parameter::Pressure => $singal_data.pressure[idx]!=0.0,
                                    crate::qc::metadata::Parameter::Windspeed => $singal_data.windspeed[idx]!=0.0,
                                    crate::qc::metadata::Parameter::WindDirection => $singal_data.winddirection[idx]!=0.0,
                                    crate::qc::metadata::Parameter::Rainfall => $singal_data.rainfull[idx]!=0.0,
                                    crate::qc::metadata::Parameter::ShortWaveDown => $singal_data.shortwavedown[idx]!=0.0,
                                    crate::qc::metadata::Parameter::ShortWaveUp => $singal_data.shortwaveup[idx]!=0.0,
                                    crate::qc::metadata::Parameter::LongWaveDown => $singal_data.longwavedown[idx]!=0.0,
                                    crate::qc::metadata::Parameter::LongWaveUp => $singal_data.longwaveup[idx]!=0.0,
                                }
                            } else  {
                                println!("Not support element: {:?}", ele);
                                is_valid = false;
                            }
                        }
                        is_valid
                    },
                    others => {
                        println!("Not support condition test: {:?}", others);
                        false
                    }
                };

                if !is_valid {
                    $singal_data.flag |= $level;
                    $all_pass = false;
                    
                }
            }
        }
    }
}
