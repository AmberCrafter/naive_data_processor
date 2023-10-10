mod utils;
mod lib;

use std::{error::Error, net::{TcpListener, TcpStream}, io::{BufReader, BufRead}, sync::{Arc, Mutex}};

use chrono::NaiveDateTime;
use clap::Parser;

use crate::{utils::cli::Cli, lib::{data_parser::{data_parser_format, data_parser, DataType}, qc_worker::QCworker}};

fn event_handler(worker: Arc<Mutex<QCworker>>, data_format:&Vec<&str>, msg: &str) {
  let data = if msg.starts_with("F=") {
    // server register formation;
    //  ex. F=0,xxx,xxx,xxx => format 0 (default pattern)
    let offset = msg.find(|x| x==',').expect("Invalid data format, missing data");
    data_parser_format(data_format, &msg[offset+1..])
  } else {
    data_parser(msg)
  };
  // println!("{:?}", data);

  // get datetime
  let mut datetime = NaiveDateTime::default();
  for val in data.iter() {
    match val.1 {
      DataType::Datetime(dt) => {
        datetime = dt.clone();
        break;
      }
      _ => {}
    }
  }

  if let Ok(mut w) = worker.lock() {
    for (target, data) in data.iter() {
      match data {
        DataType::Datetime(_) => {},
        DataType::Float(_) | DataType::Integer(_) => {
          w.append(target, datetime, data.clone());
        },
        _ => {}
      }
    }
    // w.show("temperature");
    w.show("humidity");
  }
}

fn connection_handler(stream: TcpStream, worker: Arc<Mutex<QCworker>>, data_format: &Vec<&str>) {
  let mut buf_reader = BufReader::new(stream);

  let mut buffer = String::new();
  loop {
      buffer.clear();
      buf_reader.read_line(&mut buffer).expect("System Error");

      match buffer.trim() {
          "exit" => {
              // stream.shutdown(std::net::Shutdown::Both).expect("Stream shutdown failed");
              break;
          },
          msg => {
              // println!("{}", msg);
              event_handler(worker.clone(), data_format, msg);
          }
      }
  }
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
  let mut args = Cli::parse();
  println!("cli args: {:?}", args);
  let qc_level = args.level.unwrap_or(usize::MAX);
  let data_format = args.format.split(',').collect::<Vec<_>>();

  let qcworker = Arc::new(Mutex::new(QCworker::new(&data_format)));

  let listener = TcpListener::bind("127.0.0.1:12345")?;
  for stream in listener.incoming() {
      let stream = stream.unwrap();
      connection_handler(stream, qcworker.clone(), &data_format);
  }

  // let buffer = data_parser(&args.data);

  Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn testcase1() {
        let qc = "1234,qwer";
        // println!("{:?}", qc.split_at(qc.find(|x|x==',').unwrap()+1));
        println!("{:?}", &qc[4..]);
    }
}