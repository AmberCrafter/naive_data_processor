use std::path::PathBuf;

use clap::Parser;

#[derive(Debug,Parser)]
pub struct Cli {
    pub iuput_file: PathBuf,
    pub output_file: PathBuf,
    pub level: Option<usize>,
}