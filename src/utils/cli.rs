
use chrono::NaiveDateTime;
use clap::Parser;

#[derive(Debug,Parser)]
pub struct Operations {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    Daemon(DaemonOptions),
    Qc(QcOptions),
}

#[derive(Debug, Parser)]
pub struct DaemonOptions {
    #[clap(long)]
    pub port: Option<usize>,
}

#[derive(Debug, Parser)]
pub struct QcOptions {
    #[clap(short, long)]
    pub protocol: Option<usize>,
    #[clap(short, long)]
    pub data: String,
    #[clap(short, long, default_value_t = false)]
    pub save: bool,
}