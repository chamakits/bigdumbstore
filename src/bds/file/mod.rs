use std::string::String;
use std::fs::File;
use std::error::Error;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::Stdin;
use super::file;

//S Open file
//TODO do this in a smart way
//const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v6_store";
//const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v6_store_rust";
const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v6_store_rust_struct";
const BUFF_SIZE: usize = 1024;

//TODO change everything in this file from i32 to i64
pub fn open_kv_file_read() -> File {
    let file = match File::open(KV_FILE) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open for reading {}: {}", KV_FILE,
                           Error::description(&why)),
        Ok(file) => file,
    };
    return file;
}

pub fn open_kv_file_write() -> File {
    let file = match OpenOptions::new().write(true).append(true).open(KV_FILE) {
        Err(why) => panic!("couldn't open for writing {}: {}", KV_FILE,
                           Error::description(&why)),
        Ok(file) => file,
    };
    return file;
}
//E Open file

//S converting to BdsFile
pub struct BdsFile<'a> {
    bds_file: &'a mut File
}
impl<'a> BdsFile<'a> {
    pub fn new( file:&'a mut File) -> BdsFile {
        BdsFile {
            bds_file: file
        }
    }

    //TODO change all the seek/read stuff to just one method each.
    //TODO have this return a value instead of printing out
    pub fn find_value_by_key(&mut self, key_to_find: &str) -> Option<String> {
        let file_mut = &mut self.bds_file;
        file::seek_end(file_mut);
        let mut is_key_found = false;
        let mut option_val:Option<String> = Option::None;
        while !is_key_found {

            debug!("About to seek key_size");
            let pos = file::seek_key_size(file_mut);

            if pos == 0 {
                //TODO print to error
                error!("Error! It seems this file is malformed, and only contains size for a first key");
            }
            let key_size = file::read_size(file_mut);
            debug!("Key size:{}", key_size);

            file::seek_value_size_post_read_key_size(file_mut);
            let value_size = file::read_size(file_mut);
            debug!("Value size:{}", value_size);

            file::seek_key(file_mut, key_size);
            let key = file::read_key(file_mut, key_size);
            debug!("Key: {}", key);

            let key_check = &key;
            is_key_found = key_to_find == key_check;

            debug!("Comparing {} == {}?: {}", key_to_find, key_check, is_key_found);
            debug!("To find bytes: {:?} ; key bytes: {:?}", key_to_find.to_string().into_bytes(), key_check.to_string().into_bytes());

            let position_of_next_key = file::seek_value(file_mut, value_size, key_size);
            debug!("Seeked");

            if is_key_found {
                //OLD-TODO this minus one I think comes because the bds-c writer i think is writing some new lines, so if I fix this, I should probably eventually remove this minus one eventually.
                //TODO Further investigation, it seems when you pipe values or use echo in to a command, it inclued a new line, which the bds-c writes in.
                let value_found = file::read_key(file_mut, value_size-1);
                debug!("Value found:'{}'", value_found);
                //println!("{}", value_found);
                //option_val = Option::Some(value_found.to_string());
                option_val = Option::Some(value_found);
                break;
            } else if position_of_next_key == 0 {
                option_val = Option::None;
                break;
            }
        };
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

    fn write_key_value(&mut self, key: &str, value:&str, stdin_read_size:usize) {
        let to_write = format!(
            "{}{}{:03}{:03}",
            value, key,
            stdin_read_size, key.len());

        debug!("to_write = {}", to_write);

        match self.bds_file.write_all(&to_write.into_bytes()) {
            Err(why) => panic!("Could not write value. Err: [{}]",why),
            Ok(wrote) => wrote
        }
    }
}
//E converting to BdsFile

//S Functions used for reading
pub fn seek_end(file: &mut File) {
    //let seek_res = file.seek(SeekFrom::End(0));
    match file.seek(SeekFrom::End(0)) {
        Err(why) => panic!("Could not seek to end of file. Err:{}",why),
        _ => {}
    }
}

pub fn seek_key_size(file: &mut File) -> u64 {
    match file.seek(SeekFrom::Current(-3)) {
        Err(why) => panic!("Could not seek to look for size. Err:{}",why),
        Ok(pos) => pos
    }
}

pub fn read_size(file: &mut File) -> i32 {
    let mut size_buffer = [0; 3];
    match file.read(&mut size_buffer) {
        Err(why) => panic!("Could not read size bytes. Err:{}",why),
        _ => {}
    }
    let res = match String::from_utf8_lossy(&mut size_buffer).to_mut().parse::<i32>() {
        Err(why) => panic!("Could not conver size to read to int. Err:{}", why),
        Ok(size_read) => size_read
    };
    return res
}

pub fn seek_value_size_post_read_key_size(file: &mut File) -> u64 {
    match file.seek(SeekFrom::Current(-6)) {
        Err(why) => panic!("Could not seek to look for value size. Err:{}",why),
        Ok(pos) => pos
    }
}

pub fn seek_key(file: &mut File, key_size: i32) -> u64 {
    match file.seek(SeekFrom::Current(-(key_size + 3) as i64 )) {
        Err(why) => panic!("Could not seek to look for value size. Err:{}",why),
        Ok(pos) => pos
    }
}

pub fn read_key(file: &mut File, key_size: i32) -> String {
    //TODO this is limited to only read data that fits in size. Improve
    let mut key_buffer = [0; BUFF_SIZE];
    let mut file_take = file.take(key_size as u64);
    match file_take.read(&mut key_buffer) {
        Err(why) => panic!("Could not read size bytes. Err:{}",why),
        Ok(x) => {
            debug!("File read:{}", x)
        }
    }
    let mut res = String::from_utf8_lossy(&mut key_buffer).into_owned();
    res.truncate(key_size as usize);

    return res
}

pub fn seek_value(file: &mut File, value_size:i32, key_size:i32) -> u64 {
    match file.seek(SeekFrom::Current( -(value_size + key_size) as i64)) {
        Err(why) => panic!("Could not seek value. Err:{}",why),
        Ok(pos) => pos
    }
}
//E Functions used for reading

