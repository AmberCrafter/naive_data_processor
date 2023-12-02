mod lib;
mod utils;

use std::{
    error::Error,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex}, collections::HashMap,
};

use chrono::NaiveDateTime;
use clap::Parser;

use crate::{
    lib::{
        data_parser::{data_parser_key_value, data_parser_format, DataType},
        qc_worker::{QCworker, FormationTable},
    },
    utils::cli::{Operations, Command::*},
};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let oper = Operations::parse();

    match oper.command {
        Daemon(opts) => {
            println!("daemon: {:?}", opts);
            todo!()
        },
        Qc(opts) => {
            println!("qc: {:?}", opts);
            let mut qc = QCworker::new(HashMap::new());

            let raw_data = if let Some(fidx) = opts.protocol {
                format!("F{fidx},{}", opts.data)
            } else {
                opts.data
            };

            qc.handler(&raw_data);

            qc.show_report();

            qc.set_database("database/dummy.db");
            qc.save().unwrap();
        }
    }


    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {}
}
