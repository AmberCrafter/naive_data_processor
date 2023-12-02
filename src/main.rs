mod lib;
mod utils;

use std::{error::Error, collections::HashMap};

use clap::Parser;

use crate::{
    lib::qc_worker::QCworker,
    utils::cli::{Command::*, Operations},
};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let oper = Operations::parse();

    match oper.command {
        Daemon(opts) => {
            println!("daemon: {:?}", opts);
            todo!()
        }
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
