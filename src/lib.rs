extern crate clap;

use clap::{App,Arg};
use std::result::Result;


//get_args gets the arguments from users
fn get_args() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .version_short("v")
                .arg(
                    Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .takes_value(true)
                    .help("name of the input files or directories")
                ).get_matches();

}


fn run() -> Result<()> {

   Ok(()) 
}