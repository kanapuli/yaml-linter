#![warn(rust_2018_idioms)]

use crate::common::*;

use clap::{App, Arg, ArgMatches};
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

mod common;
//Warning is the lint warnings
pub struct Warning {}

//get_args gets the arguments from users
fn get_args(current_dir: &OsStr) -> ArgMatches {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version_short("v")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("file names or directory name to check")
                .index(1)
                .takes_value(true)
                .default_value_os(current_dir)
                .multiple(true)
                .required(true),
        )
        .get_matches();
    return matches;
}

fn run() -> Result<Vec<Warning>, Box<dyn Error>> {
    let warnings = Vec::new();
    //set default directory
    let current_directory = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => return Err(Box::new(e)),
    };
    let current_directory = current_directory.as_os_str();

    let args = get_args(current_directory);
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut files: Vec<File> = Vec::new();

    if let Some(inputs) = args.values_of("input") {
        dirs = inputs
            .clone()
            .filter_map(|s| fs::canonicalize(s).ok())
            .filter(|p| p.is_dir())
            .collect();

        files = inputs
            .filter_map(|s| fs::canonicalize(s).ok())
            .filter(|p| p.is_file())
            .filter_map(|f| File::from(f))
            .collect();
    };
    Ok(warnings)
}
