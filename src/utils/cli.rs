
use chrono::NaiveDateTime;
use clap::Parser;

#[derive(Debug,Parser)]
pub struct Cli {
    // pub time: NaiveDateTime,
    // pub data: String,
    pub format: String,
    pub level: Option<usize>,
}

// impl Default for Cli {
//     fn default() -> Self {
//         Self { iuput_file: (), output_file: (), error_file: (), level: () }
//     }
// }

