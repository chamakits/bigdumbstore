use std::env;
mod mode;

fn main() {

    //
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    let run_mode = mode::determine_mode(args);
    println!("Run mode is: {:?}", run_mode);
}
