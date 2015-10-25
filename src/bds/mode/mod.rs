#[derive(Debug)]
pub enum Mode {
    //TODO maybe change to be a key only instead of the whole vector
    Read(Vec<String>, Option<String>),
    Write(Vec<String>, Option<String>),
    Invalid(Vec<String>),
}

pub fn determine_mode(arguments: Vec<String>) -> Mode {
    let arg = arguments.get(1).unwrap().split_at(1);
    debug!("Arg split: {:?}", arg);
    match arg.0 {
        "g" => {
            let mut read_val:Vec<String> = arguments.to_vec();
            read_val.remove(0);
            read_val.remove(0);
            {
                let _path_kv_file = arg.1.to_string();
                let path_kv_file = if _path_kv_file.len() > 0 {
                    Option::Some(_path_kv_file)
                } else {
                    Option::None
                };

                Mode::Read(read_val, path_kv_file)
            }
        },
        "p" => {
            let mut read_val:Vec<String> = arguments.to_vec();
            read_val.remove(0);
            read_val.remove(0);
            //S
            {
                let _path_kv_file = arg.1.to_string();
                let path_kv_file = if _path_kv_file.len() > 0 {
                    Option::Some(_path_kv_file)
                } else {
                    Option::None
                };

                Mode::Write(read_val, path_kv_file)
            }            
            //E
        },
        x => {
            error!("Given argument is Invalid: {}", x);
            Mode::Invalid(arguments.to_vec())
        },
    }
}
