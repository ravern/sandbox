#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate failure;
extern crate fern;
extern crate yu;

use clap::{App, Arg};
use failure::Error;
use fern::Dispatch;
use log::LevelFilter;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::process;

type Result<T> = ::std::result::Result<T, Error>;

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
  let matches = App::new(crate_name!())
    .about("The Yu programming language")
    .version(crate_version!())
    .author(crate_authors!())
    .arg(
      Arg::with_name("verbose")
        .help("Enable verbose output")
        .short("v")
        .long("verbose")
        .takes_value(false),
    ).arg(
      Arg::with_name("path")
        .help("Path of the source file to run")
        .value_name("PATH"),
    ).get_matches();

  let verbose = matches.is_present("verbose");
  if let Err(err) = logger(verbose).apply() {
    eprintln!("error: {}", err);
    process::exit(63);
  }

  let path = matches.value_of("path");
  run(path)
}

fn run(path: Option<&str>) -> Result<()> {
  match path {
    Some(path) => run_file(path),
    None => run_interactive(),
  }
}

fn run_file(path: &str) -> Result<()> {
  info!("Running file at {}!", path);
  Ok(())
}

fn run_interactive() -> Result<()> {
  println!("{} {}", crate_name!(), crate_version!());
  println!("{}", crate_authors!());

  loop {
    print!("> ");
    flush()?;

    let program = match read_line() {
      Some(line) => line?,
      None => return Ok(()),
    };

    println!("{}", program);
  }
}

fn logger(verbose: bool) -> Dispatch {
  let level = if verbose {
    LevelFilter::Info
  } else {
    LevelFilter::Warn
  };

  Dispatch::new()
    .level(level)
    .chain(::std::io::stderr())
    .format(move |out, message, record| {
      let level = format!("{}", record.level()).to_lowercase();
      out.finish(format_args!("{}: {}", level, message))
    })
}

fn flush() -> Result<()> {
  io::stdout().flush()?;
  Ok(())
}

fn read_line() -> Option<Result<String>> {
  let stdin = io::stdin();
  let mut iter = stdin.lock().lines();
  iter.next().map(|line| Ok(line?))
}
