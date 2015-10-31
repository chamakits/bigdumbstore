#[derive(Debug)]
pub enum Mode<'a> {
    // TODO maybe change to be a key only instead of the whole vector
    Read(&'a Vec<String>, Option<String>),
    Write(&'a Vec<String>, Option<String>),
    JunkWrite(&'a Vec<String>),
    Server(&'a Vec<String>),
    Invalid(&'a Vec<String>),
}

pub fn determine_mode<'a>(arguments: &'a mut Vec<String>) -> Mode {

    let arg = {
        let _arg = arguments.get_mut(1).unwrap().split_at(1);
        (_arg.0, _arg.1)
    };

    
    debug!("Arg split: {:?}", arg);
    match arg.0 {
        "g" => {
            //let mut read_val: Vec<String> = arguments.to_vec();
            let read_val = arguments;
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
        }
        "p" => {
            //let mut read_val: Vec<String> = arguments.to_vec();
            let read_val = arguments;
            read_val.remove(0);
            read_val.remove(0);
            // S
            {
                let _path_kv_file = arg.1.to_string();
                let path_kv_file = if _path_kv_file.len() > 0 {
                    Option::Some(_path_kv_file)
                } else {
                    Option::None
                };

                Mode::Write(read_val, path_kv_file)
            }
            // E
        }
        "j" => {
            //let mut read_val: Vec<String> = arguments.to_vec();
            let read_val = arguments;
            read_val.remove(0);
            read_val.remove(0);

            Mode::JunkWrite(read_val)
        }
        "s" => {
            //let mut read_val: Vec<String> = arguments.to_vec();
            let read_val = arguments;
            read_val.remove(0);
            read_val.remove(0);
            Mode::Server(read_val)
        }
        x => {
            error!("Given argument is Invalid: {}", x);
            Mode::Invalid(arguments)
        }
    }
}
