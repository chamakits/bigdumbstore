#[macro_use]
extern crate log;
extern crate env_logger;
extern crate bit_vec;
extern crate tempdir;

use std::env;
mod bds;

use bds::runner::Mode;
use bds::runner;

fn main() {
    setup_logging();
    let args: Vec<String> = env::args().collect();
    debug!("Args: {:?}", args);
    let run_mode = runner::determine_mode(args);
    debug!("Run mode is: {:?}", run_mode);
    match run_mode {
        Mode::Read(args, path) => runner::reading(args, path),
        Mode::Write(args, path) => runner::writing(args, path),
        Mode::JunkWrite(args) => runner::junk_writing(args),
        Mode::Nothing => {
            do_nothing()
        },
        x => panic!("Parameter given is invalid. Args:{:?}", x),
    }
}

pub fn setup_logging() {
    env_logger::init().unwrap();
}

//Created this function to have it picked up in the tests in hope of it showing up in 'coveralls'
fn do_nothing(){
    debug!("Doing nothing");
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    #[test]
    fn test_do_nothing() {
        super::setup_logging();
        super::do_nothing();
    }
}
