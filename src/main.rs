#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod action;
pub mod app;
pub mod cli;
pub mod components;
pub mod config;
pub mod mode;
pub mod tui;
pub mod utils;
pub mod domain;

use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;

use crate::{
  app::App,
  utils::{initialize_logging, initialize_panic_handler, version},
};
use crate::domain::library;
use crate::domain::library::Library;

async fn tokio_main() -> Result<()> {
  initialize_logging()?;

  initialize_panic_handler()?;

  let args = Cli::parse();
  let library = args.library.and_then(init_library_from_arg).unwrap_or(Library{ audiobooks: Vec::new(), file: None });
  let mut app = App::new(args.tick_rate, args.frame_rate, &library)?;
  app.run().await?;
  Ok(())
}

fn init_library_from_arg(filename: String) -> Option<Library> {
  let temp = library::parse_library_json(filename);
  match temp {
    Ok(lib) => {
      return Some(lib);
    }
    Err(error) => {
      panic!("The given library file could not be parsed, because: {}", error);
    }
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  if let Err(e) = tokio_main().await {
    eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
    Err(e)
  } else {
    Ok(())
  }
}
