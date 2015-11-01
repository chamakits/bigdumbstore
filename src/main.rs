#[macro_use]
extern crate log;
extern crate env_logger;
extern crate bit_vec;
extern crate tempdir;

use std::env;
mod bds;

use bds::mode;
use bds::mode::Mode;
use bds::runner;

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    debug!("Args: {:?}", args);
    let run_mode = mode::determine_mode(args);
    debug!("Run mode is: {:?}", run_mode);
    match run_mode {
        Mode::Read(args, path) => runner::reading(args, path),
        Mode::Write(args, path) => runner::writing(args, path),
        Mode::JunkWrite(args) => runner::junk_writing(args),
        x => panic!("Parameter given is invalid. Args:{:?}", x),
    }

}
