#[macro_use]
extern crate log;
extern crate env_logger;
extern crate log4rs;
extern crate bit_vec;
extern crate tempdir;

use std::env;
mod bds;

use bds::runner::Mode;
use bds::runner;

fn main() {
    //env_logger::init().unwrap();
    //log4rs::init_file("config/log.toml", Default::default()).unwrap();
    //log4rs::init_config(setup_logging()).unwrap();
    setup_logging();
    info!("main started");
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

/*
use log::{LogLevelFilter};
use log4rs::config::{Root, Config, Appender, ConfigBuilder, AppenderBuilder};
use log4rs::appender::{ConsoleAppender};
*/
pub fn setup_logging() {
    env_logger::init().unwrap();
    /*
    let root = Root::builder(LogLevelFilter::Debug).appender("stderr".to_string());
    let console = Box::new(ConsoleAppender::builder().build());
    let config = Config::builder(root.build())
        .appender(Appender::builder("stderr".to_string(), console).build());
    log4rs::init_config(config.build().unwrap()).unwrap();
    */
}

//Created this function to have it picked up in the tests in hope of it showing up in 'coveralls'
fn do_nothing(){
    debug!("Doing nothing");
}

#[cfg(test)]
mod tests {
    extern crate log4rs;
    extern crate env_logger;
    use std::default::Default;
    #[test]
    fn test_do_nothing() {
        //log4rs::init_file("config/log.toml", Default::default()).unwrap();
        //env_logger::init().unwrap();
        super::setup_logging();
        super::do_nothing();
    }
}
