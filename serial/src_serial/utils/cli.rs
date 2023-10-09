use std::path::PathBuf;

use clap::Parser;

#[derive(Debug,Parser)]
pub struct Cli {
    pub iuput_file: PathBuf,
    pub output_file: PathBuf,
    pub error_file:Option<PathBuf>,
    pub level: Option<usize>,
}

// impl Default for Cli {
//     fn default() -> Self {
//         Self { iuput_file: (), output_file: (), error_file: (), level: () }
//     }
// }