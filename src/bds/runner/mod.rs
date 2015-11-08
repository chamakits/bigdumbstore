use super::file;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::char;
use std::io::prelude::*;

// TODO do this in a smart way
const KV_FILE: &'static str = "/home/chamakits/.config/bigdumbstore/.v0.0.1_store";
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Mode {
    // TODO maybe change to be a key only instead of the whole vector
    Read(Vec<String>, Option<String>),
    Write(Vec<String>, Option<String>),
    JunkWrite(Vec<String>),
    Server(Vec<String>),
    Nothing,
    Invalid(Vec<String>),
}

pub fn determine_mode(arguments: Vec<String>) -> Mode {
    let arg = arguments.get(1).unwrap().split_at(1);
    debug!("Arg split: {:?}", arg);
    match arg.0 {
        "g" => {
            let mut read_val: Vec<String> = arguments.to_vec();
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
            let mut read_val: Vec<String> = arguments.to_vec();
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
            let mut read_val: Vec<String> = arguments.to_vec();
            read_val.remove(0);
            read_val.remove(0);

            Mode::JunkWrite(read_val)
        }
        "s" => {
            let mut read_val: Vec<String> = arguments.to_vec();
            read_val.remove(0);
            read_val.remove(0);
            Mode::Server(read_val)
        }
        "0" => {
            Mode::Nothing
        }
        x => {
            error!("Given argument is Invalid: {}", x);
            Mode::Invalid(arguments.to_vec())
        }
    }
}

fn create_file_if_not_exist(kv_file_path_str: &str) {
    let kv_file_path = Path::new(kv_file_path_str);

    debug!("kv_file_path: {:?}", kv_file_path);

    match fs::metadata(kv_file_path_str) {
        Err(_) => {
            let kv_path_dirs_created = create_directories_if_needed(kv_file_path.parent().unwrap());
            let kv_path_joined = Path::new(&kv_path_dirs_created)
                .join(kv_file_path.file_name().unwrap());
            debug!("kv_path_dirs_created: {:?}", kv_path_joined);
            match fs::File::create(kv_path_joined) {
                Err(why) => panic!("Couldn't create file. Err: {}, for path: {}",
                                   why,
                                   kv_file_path_str),
                _ => {}
            }
        }
        _ => {}
    }
}

fn path_with_curly_to_abs(path_maybe_with_curly: &str) -> PathBuf {
    let resolve_path_for_home = match path_maybe_with_curly.split_at(1) {
        ("~", x) => {
            let rest_of_path = x.split_at(1).1;
            debug!("Found home directory, file:{}, {:?}",
                   x,
                   env::home_dir().unwrap().join(Path::new(rest_of_path)));
            env::home_dir().unwrap().join(Path::new(rest_of_path))
        }
        _ => {
            debug!("Not a home directory specified");
            Path::new(path_maybe_with_curly).to_path_buf()
        }
    };
    resolve_path_for_home
}

fn create_directories_if_needed(path: &Path) -> String {
    debug!("create_directories_if_needed: {}", path.to_str().unwrap());
    let path_str = path.to_str().unwrap();

    if path_str.len() == 0 {
        return path_str.to_string();
    }

    let resolve_path_for_home = path_with_curly_to_abs(path_str);
    debug!("resolve_path_for_home: {:?}", resolve_path_for_home);

    match fs::create_dir_all(&resolve_path_for_home) {
        Err(why) => panic!("Couldn't create directory path {:?}: error:{}", path, why),
        Ok(_) => {}
    }

    return resolve_path_for_home.to_str().unwrap().to_string();
}

pub fn reading(read_args: Vec<String>, path: Option<String>) -> Option<String> {

    let mut path_str: String = match path {
        Option::Some(_path_str) => _path_str.to_string(),
        Option::None => KV_FILE.to_string(),
    };

    path_str = path_with_curly_to_abs(&path_str).to_str().unwrap().to_string();

    debug!("Will be reading with args: {:?}, path: {:?}",
           read_args,
           path_str);
    create_file_if_not_exist(&path_str);

    let key_to_find = read_args.get(0).unwrap();

    let mut bds = file::BdsFile::new_read(&path_str);

    let value_found = bds.find_value_by_key_from_beginning(key_to_find);

    value_found
}

const DEFAULT_KEY: &'static str = "default";
pub fn junk_writing(write_args: Vec<String>) {
    let path = "JunkKVFile";
    create_file_if_not_exist(path);
    let root_key = match write_args.get(0) {
        Some(key) => format!("{}",key),
        None => format!("{}",DEFAULT_KEY),
    };
    let mut bds = file::BdsFile::new_write(path);

    let mut bigger_str = "".to_string();
    let mut to_write:String;
    
    let a = 'a' as u32;
    let outer = 25;
    let inner = 25;
    for i in 0..outer {
        for j in 0..inner {
            to_write = format!("Smaller!_BEFORE_{}{}_AFTER", char::from_u32(i+a).unwrap(), char::from_u32(j+a).unwrap());
            bigger_str = bigger_str + &to_write;
            let key = format!("{}_key_{}_{}", root_key, inner, outer);
            bds.write_to_key_dynamic(&key, &to_write);
            let bigger_again = "Bigger!".to_string() + &bigger_str;
            bds.write_to_key_dynamic(&key, &bigger_again);
        }
    }
    
}

pub fn writing(write_args: Vec<String>, path: Option<String>, read_from: &mut Read) {

    let mut path_str: String = match path {
        Option::Some(_path_str) => _path_str.to_string(),
        Option::None => KV_FILE.to_string(),
    };

    path_str = path_with_curly_to_abs(&path_str).to_str().unwrap().to_string();

    debug!("Will be writing with args: {:?}, path: {:?}",
           write_args,
           path_str);

    create_file_if_not_exist(&path_str);
    let key_to_write = write_args.get(0).unwrap();
    //let mut stdin = &mut io::stdin();

    let mut bds = file::BdsFile::new_write(&path_str);
    //bds.write_to_key_from_stdin(key_to_write, stdin);
    bds.write_to_key_from_stdin(key_to_write, read_from);
}
#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use super::super::tests::*;
    use std::fs;
    
    #[macro_export]
    macro_rules! assign_do_nothing {
        ( $( $x:expr ),* ) => {
            {
                $(
                    let _curr = $x;
                    )*

            }
        };
    }

    #[test]
    fn mock_forcing_functions_compiled() {
        assign_do_nothing![
            super::determine_mode, 
            super::junk_writing, 
            super::reading, 
            super::writing, 
            super::path_with_curly_to_abs, 
            super::create_directories_if_needed];
    }

    #[test]
    fn test_determine_mode() {
        //reading
        //S
        super::super::super::setup_logging();
        let args: Vec<String> = vec!["ignored", "g_kvf", "given-key"].iter()
            .map(|x| x.to_string()).collect();
        let mode = super::determine_mode(args.to_vec());
        //let expected = super::Mode::Read(args, Option::Some("_kvf".to_string()));
        let exp = super::Mode::Read(vec!["given-key".to_string()], Option::Some("_kvf".to_string()));
        {info!("expected: '{:?}', mode: '{:?}'", exp, mode);}
        assert_eq!(exp, mode);
        //E

        //writing
        //S
        super::super::super::setup_logging();
        let args: Vec<String> = vec!["ignored", "p_kvf", "given-key"].iter()
            .map(|x| x.to_string()).collect();
        let mode = super::determine_mode(args.to_vec());
        let exp = super::Mode::Write(vec!["given-key".to_string()], Option::Some("_kvf".to_string()));
        {info!("expected: '{:?}', mode: '{:?}'", exp, mode);}
        assert_eq!(exp, mode);
        //E

        //junk_mode
        //S
        super::super::super::setup_logging();
        let args: Vec<String> = vec!["ignored", "j", "key"].iter()
            .map(|x| x.to_string()).collect();
        let mode = super::determine_mode(args.to_vec());
        let exp = super::Mode::JunkWrite(vec!["key".to_string()]);
        {info!("expected: '{:?}', mode: '{:?}'", exp, mode);}
        assert_eq!(exp, mode);
        //E
    }

    #[test]
    fn test_create_file_if_not_exist() {
        let tmp_dir = TempDir::new("bds_kv_dir").unwrap();
        let tmp_path_str = &temp_file_path_string(&tmp_dir);

        {
            let file_found = match fs::metadata(tmp_path_str){
                Err(_) => Option::None,
                Ok(_) => Option::Some(false),
            };
            assert_eq!(Option::None, file_found);
        }

        super::create_file_if_not_exist(tmp_path_str);

        let attr = match fs::metadata(tmp_path_str){
            Ok(curr) => curr,
            Err(why) => panic!("Couln't check file:{}", why),
        };
        let file_exists = attr.is_file();

        assert_eq!(true, file_exists);
    }

    use std::io::Cursor;
    #[test]
    fn test_writing_and_reading() {

        //Getting value to read
        let write_str = "This is what I'm writing";
        let bytes = write_str.to_string().into_bytes();
        let mut cursor_to_read = Cursor::new(bytes);

        //Key
        let key = vec!["key_given".to_string()];

        //Path
        let _tmp_dir = TempDir::new("bds_kv_dir");
        let tmp_dir = _tmp_dir.unwrap();
        let tmp_path_str = temp_file_path_string(&tmp_dir);


        super::writing(
            key.to_vec(), 
            Option::Some(tmp_path_str.to_string()), 
            &mut cursor_to_read);
        
        let other_key = vec!["other_key_given".to_string()];
        super::writing(
            other_key.to_vec(), 
            Option::Some(tmp_path_str.to_string()), 
            &mut cursor_to_read);

        let val_read = super::reading(
            key.to_vec(), 
            Option::Some(tmp_path_str.to_string()));
        assert_eq!(write_str.to_string(), val_read.unwrap());
    }
}
