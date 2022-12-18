extern crate byteorder;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate enum_primitive_derive;
#[macro_use]
extern crate failure;
extern crate fern;
#[macro_use]
extern crate log;
extern crate num_traits;

mod core;
mod instruction;
mod logger;
mod result;
mod stack;

use clap::{App, Arg};
use core::Core;
use result::Result;
use std::fs;
use std::process;

fn main() {
  match try_main() {
    Ok(_) => process::exit(0),
    Err(err) => {
      error!("{}", err);
      process::exit(1);
    }
  }
}

fn try_main() -> Result<()> {
  let matches = App::new("kato")
    .about("Toy virtual machine written in Rust")
    .version(crate_version!())
    .author(crate_authors!())
    .arg(
      Arg::with_name("path")
        .help("Program to execute")
        .value_name("PROGRAM")
        .required(true),
    ).arg(
      Arg::with_name("verbose")
        .help("Enable verbose output")
        .short("v")
        .long("verbose")
        .takes_value(false),
    ).get_matches();

  let path = matches.value_of("path").unwrap();
  let verbose = matches.is_present("verbose");

  logger::init(verbose).unwrap();

  let program_bytes = fs::read(path)?;
  info!("Opened program {}", path);
  Core::new(program_bytes).execute()
}
