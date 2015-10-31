#[derive(Debug)]
pub enum Mode<'a> {
    // TODO maybe change to be a key only instead of the whole vector
    Read(&'a [String], Option<String>),
    Write(&'a [String], Option<String>),
    JunkWrite(&'a [String]),
    Server(&'a [String]),
    Invalid(&'a [String]),
}

pub fn determine_mode<'a>(arguments: &'a mut Vec<String>) -> Mode {

    let (first_arg, other_args) =  {
        let (_1, _2) = arguments.split_at(1);
        _2.split_at(1)
    };
    debug!("first_arg: {:?}, other_args: {:?}", first_arg, other_args);
    
    let arg = first_arg.get(0).unwrap().split_at(1);
    debug!("Arg split: {:?}", arg);
    let arg_command = arg.0.unwrap();
    match arg_command.as_ref() {
        "g" => {
            //let mut read_val: Vec<String> = arguments.to_vec();
            let read_val = other_args.split_at(2).1;
            {
                let _path_kv_file = arg.1;
                //*_path_kv_file + 1;
                let path_kv_file = if _path_kv_file.len() > 0 {
                    Option::Some(_path_kv_file.get(0).unwrap().to_string())
                } else {
                    Option::None
                };

                Mode::Read(read_val, path_kv_file)
            }
        }
        "p" => {
            let read_val = other_args.split_at(2).1;
            // S
            {
                let _path_kv_file = arg.1;
                let path_kv_file = if _path_kv_file.len() > 0 {
                    Option::Some(_path_kv_file.get(0).unwrap().to_string())
                } else {
                    Option::None
                };

                Mode::Write(read_val, path_kv_file)
            }
            // E
        }
        "j" => {
            //let mut read_val: Vec<String> = arguments.to_vec();
            let read_val = other_args.split_at(2).1;

            Mode::JunkWrite(read_val)
        }
        "s" => {
            //let mut read_val: Vec<String> = arguments.to_vec();
            let read_val = other_args.split_at(2).1;
            Mode::Server(read_val)
        }
        x => {
            error!("Given argument is Invalid: {}", x);
            Mode::Invalid(arguments)
        }
    }
}
