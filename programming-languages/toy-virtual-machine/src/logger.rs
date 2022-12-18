use fern::Dispatch;
use log::LevelFilter;
use result::Result;
use std::io::{stderr, stdout};

pub fn init(verbose: bool) -> Result<()> {
  let dispatch = Dispatch::new()
    .level(level(verbose))
    .chain(
      Dispatch::new()
        .filter(|metadata| metadata.level() == LevelFilter::Info)
        .chain(stdout()),
    ).chain(
      Dispatch::new()
        .filter(|metadata| metadata.level() == LevelFilter::Error)
        .chain(stderr()),
    ).format(move |out, message, record| {
      let level = format!("{}", record.level()).to_lowercase();
      out.finish(format_args!("{}: {}", level, message))
    });

  match dispatch.apply() {
    Ok(()) => Ok(()),
    Err(err) => Err(format_err!("{}", err)),
  }
}

fn level(verbose: bool) -> LevelFilter {
  if verbose {
    LevelFilter::Info
  } else {
    LevelFilter::Error
  }
}
