#![feature(test)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate bit_vec;
extern crate tempdir;
extern crate home;

use std::env;
use std::io;
mod bds;
mod tests;

use bds::runner::Mode;
use bds::runner;

fn main() {
    setup_logging();
    let args: Vec<String> = env::args().collect();
    debug!("Args: {:?}", args);
    let run_mode = runner::determine_mode(&args);
    debug!("Run mode is: {:?}", run_mode);
    match run_mode {
        Mode::Read(args, path) => {
            let value_found = runner::reading(args, path);
            if value_found.is_some() {
                println!("{}", value_found.unwrap());
            }
        },
        Mode::Write(args, path) => {
            let stdin = &mut io::stdin();
            runner::writing(args, path, stdin)
        },
        Mode::JunkWrite(args) => runner::junk_writing(args, Option::None, 40, 40),
        Mode::Nothing => {
            do_nothing()
        },
        x => panic!("Parameter given is invalid. Args:{:?}", x),
    }
}

pub fn setup_logging() {
    
    env_logger::init();
}

//Created this function to have it picked up in the tests in hope of it showing up in 'coveralls'
fn do_nothing(){
    debug!("Doing nothing");
}

