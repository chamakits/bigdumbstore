use std::string::String;
use std::fs::File;
use std::error::Error;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::Stdin;

// TODO change everything in this file from i32 to i64

// S Open file
// TODO do this in a smart way
const BUFF_SIZE: usize = 1024;
// E Open file

// S converting to BdsFile
pub struct BdsFile {
    bds_file: File,
}
const SEEK_GOTO_END: SeekFrom = SeekFrom::End(0);
const SEEK_KEY_SIZE: SeekFrom = SeekFrom::Current(-3);
const SEEK_VALUE_SIZE_POST_READ_KEY_SIZE: SeekFrom = SeekFrom::Current(-6);

impl BdsFile {
    pub fn new_read(file_path: &str) -> BdsFile {
        let file = match File::open(file_path) {
            Err(why) => panic!("couldn't open for reading {}: {}",
                               file_path,
                               Error::description(&why)),
            Ok(file) => file,
        };
        BdsFile { bds_file: file }
    }

    pub fn new_write(file_path: &str) -> BdsFile {
        let file = match OpenOptions::new().write(true).append(true).open(file_path) {
            Err(why) => panic!("couldn't open for writing {}: {}",
                               file_path,
                               Error::description(&why)),
            Ok(file) => file,
        };
        BdsFile { bds_file: file }
    }

    pub fn find_value_by_key(&mut self, key_to_find: &str) -> Option<String> {
        let file_mut = &mut self.bds_file;

        BdsFile::seek_start_of_file_fail_if_empty(file_mut, key_to_find);
        let mut is_key_found = false;
        let mut option_val: Option<String> = Option::None;
        while !is_key_found {

            let key_size = BdsFile::read_key_size(file_mut);
            let value_size = BdsFile::read_value_size(file_mut);
            let key_to_check = &BdsFile::read_key_string(file_mut, key_size);

            is_key_found = key_to_find == key_to_check;

            debug!("Comparing {} == {}?: {}",
                   key_to_find,
                   key_to_check,
                   is_key_found);

            let position_of_next_key = BdsFile::seek_value(file_mut, value_size, key_size);

            if is_key_found {
                // TODO Further investigation, it seems when you pipe values or use echo in to
                // a command, it inclued a new line, which the bds-c writes in.
                option_val = BdsFile::read_value_string_option(file_mut, value_size);
                break;
            } else if position_of_next_key == 0 {
                option_val = Option::None;
                break;
            }
        }
        option_val
    }
    
    fn seek_start_of_file_fail_if_empty(file_mut: &mut File, key_to_find: &str) {
        let pos = BdsFile::seek_with(file_mut, SEEK_GOTO_END);
        debug!("Position after reading to end: {}", pos);
        if pos == 0 {
            panic!("File found, but is empty. Cannot look for: [{}]",
                   key_to_find);
        }
    }

    fn read_key_size(file_mut: &mut File) -> i64 {
        debug!("About to seek key_size");
        let pos = BdsFile::seek_with(file_mut, SEEK_KEY_SIZE);
        if pos == 0 {
            error!("Error! It seems this file is malformed, and only contains size for a first key");
        }
        let key_size = BdsFile::read_size(file_mut);
        debug!("Key size:{}", key_size);
        return key_size;
    }

    fn read_value_size(file_mut: &mut File) -> i64 {
        BdsFile::seek_with(file_mut, SEEK_VALUE_SIZE_POST_READ_KEY_SIZE);
        let value_size = BdsFile::read_size(file_mut);
        debug!("Value size:{}", value_size);
        value_size
    }

    fn read_key_string(file_mut: &mut File, key_size: i64) -> String {
        BdsFile::seek_key(file_mut, key_size);
        let key = BdsFile::read_key(file_mut, key_size);
        debug!("Key: {}", key);
        key
    }

    fn read_value_string_option(file_mut: &mut File, value_size: i64) -> Option<String> {
        let value_found = BdsFile::read_key(file_mut, value_size - 1);
        debug!("Value found:'{}'", value_found);
        let option_val = Option::Some(value_found);
        option_val
    }

    pub fn write_to_key_from_stdin(&mut self, key: &str, stdin: &mut Stdin) {
        let mut string_in = &mut String::with_capacity(BUFF_SIZE);
        let _stdin_read_size = stdin.read_to_string(string_in);
        let stdin_read_size = _stdin_read_size.unwrap();
        string_in.truncate(stdin_read_size);

        debug!("Read from input:{}", string_in);

        self.write_key_value(key, string_in, stdin_read_size);
    }

    fn write_key_value(&mut self, key: &str, value: &str, stdin_read_size: usize) {
        let to_write = format!("{}{}{:03}{:03}", value, key, stdin_read_size, key.len());

        debug!("to_write = {}", to_write);

        match self.bds_file.write_all(&to_write.into_bytes()) {
            Err(why) => panic!("Could not write value. Err: [{}]", why),
            Ok(wrote) => wrote,
        }
    }

    fn seek_with(file: &mut File, seeker: SeekFrom) -> u64 {
        match file.seek(seeker) {
            Err(why) => panic!("Could not seek with {:?}. Err:{}", seeker, why),
            Ok(pos) => pos,
        }
    }

    fn read_size(file: &mut File) -> i64 {
        let mut size_buffer = [0; 3];
        match file.read(&mut size_buffer) {
            Err(why) => panic!("Could not read size bytes. Err:{}", why),
            _ => {}
        }
        let res = match String::from_utf8_lossy(&mut size_buffer).to_mut().parse::<i64>() {
            Err(why) => panic!("Could not conver size to read to int. Err:{}", why),
            Ok(size_read) => size_read,
        };
        return res;
    }

    fn seek_key(file: &mut File, key_size: i64) -> u64 {
        BdsFile::seek_with(file, SeekFrom::Current(-(key_size + 3)))
    }

    fn read_key(file: &mut File, key_size: i64) -> String {
        // TODO this is limited to only read data that fits in size. Improve
        let mut key_buffer = [0; BUFF_SIZE];
        let mut file_take = file.take(key_size as u64);
        match file_take.read(&mut key_buffer) {
            Err(why) => panic!("Could not read size bytes. Err:{}", why),
            Ok(x) => {
                debug!("File read:{}", x)
            }
        }
        let mut res = String::from_utf8_lossy(&mut key_buffer).into_owned();
        res.truncate(key_size as usize);

        return res;
    }

    fn seek_value(file: &mut File, value_size: i64, key_size: i64) -> u64 {
        let seeked_value =
            BdsFile::seek_with(file, SeekFrom::Current(-(value_size + key_size)));
        debug!("Seeked");
        seeked_value
    }
}
// E converting to BdsFile
