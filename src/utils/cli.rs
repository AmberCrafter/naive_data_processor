
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
    #[clap(long, default_value_t = 50500)]
    pub port: usize,
}

#[derive(Debug, Parser)]
pub struct QcOptions {
    #[clap(short, long)]
    pub protocol: Option<u32>,
    #[clap(short, long)]
    pub data: String,
    #[clap(short, long, default_value_t = false)]
    pub save: bool,

    #[clap(long)]
    pub ip: Option<String>,
    #[clap(long, default_value_t = 50500)]
    pub port: usize,
}