extern crate clap;

use clap::{App,Arg};
use std::env;
use std::error::Error;
use std::ffi::OsStr;

//Warning is the lint warnings
pub struct Warning{}

//get_args gets the arguments from users
fn get_args(current_dir: &OsStr) {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .version_short("v")
                .arg(
                    Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .takes_value(true)
                    .help("file names or directory name to check")
                    .default_value_os(current_dir)
                ).get_matches();

}


fn run() -> Result<Vec<Warning>, Box<dyn Error>> {
   //set default directory
   let current_directory = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => return Err(Box::new(e))
   };
   let current_directory = current_directory.as_os_str();
   
   let args =  get_args(current_directory);
}