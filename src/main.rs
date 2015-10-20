use std::env;
mod bds;

use bds::mode;
use bds::mode::{Mode};
use bds::runner;

fn main() {

    //
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    let run_mode = mode::determine_mode(args);
    println!("Run mode is: {:?}", run_mode);
    match run_mode {
        Mode::Read(args) => runner::reading(args),
        x => panic!("Parameter given is invalid. Args:{:?}", x),
    }
    
}
