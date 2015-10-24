#[derive(Debug)]
pub enum Mode {
    //TODO maybe change to be a key only instead of the whole vector
    Read(Vec<String>),
    Write(Vec<String>),
    Invalid(Vec<String>),
}

pub fn determine_mode(arguments: Vec<String>) -> Mode {
    match arguments.get(1).unwrap().as_ref() {
        "g" => {
            let mut read_val:Vec<String> = arguments.to_vec();
            read_val.remove(0);
            read_val.remove(0);
            Mode::Read(read_val)
        },
        "p" => {
            let mut read_val:Vec<String> = arguments.to_vec();
            read_val.remove(0);
            read_val.remove(0);
            Mode::Write(read_val)
        },
        x => {
            error!("Given argument is Invalid: {}", x);
            Mode::Invalid(arguments.to_vec())
        },
    }
}
