mod qc;
mod utils;

use std::{rc::Rc, cell::RefCell, error::Error, path::Path, fs::File, io::{LineWriter, Write}};

use clap::Parser;

use crate::{qc::metadata::{read_data, Header}, utils::cli::Cli};

fn main() -> Result<(), Box<dyn Error + 'static>> {
  let mut args = Cli::parse();
  println!("cli args: {:?}", args);
  let mut data = read_data(args.iuput_file);
  let qc_level = args.level.unwrap_or(usize::MAX);

  // let mut data = read_data(Path::new("./data/data.csv").to_path_buf());
  // println!("{:#?}", data);

  let qc_level = usize::MAX;

  let mut all_pass = true;
  {
    qc::qc_level0::main(&mut data, "./config/level0.toml")?;

    if qc_level >= 1 {
      all_pass &= qc::qc_level1::main(&mut data, "./config/level1.toml")?;
    }
  }
  println!("Level: {:?}\tAll pass: {:?}", 1, all_pass);
  // println!("Data: {:?}", data);

  // let file = File::create("./data/result.csv")?;
  let file = File::create(args.output_file)?;
  let errfile = File::create(args.error_file.unwrap_or("./data/error.csv".into()))?;
  let mut file_writer = LineWriter::new(file);
  let mut errfile_writer = LineWriter::new(errfile);
  let header = Header::gen_header();
  let buf = header.join(",");
  file_writer.write_all(buf.as_bytes())?;
  file_writer.write_all(b"\n")?;
  errfile_writer.write_all(buf.as_bytes())?;
  errfile_writer.write_all(b"\n")?;
  for ele in data {
    let buf = ele.to_vec().join(",");
    if ele.flag.bits() > 0 {
      // println!("{:?}", ele.datetime);
      errfile_writer.write_all(buf.as_bytes())?;
      errfile_writer.write_all(b"\n")?;
    }
    file_writer.write_all(buf.as_bytes())?;   
    file_writer.write_all(b"\n")?;   
  }
  file_writer.flush()?;

  Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
