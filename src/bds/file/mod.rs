use std::string::String;
use std::fs::File;
use std::error::Error;
use std::io::SeekFrom;
use std::io::prelude::*;

//TODO do this in a smart way
const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v6_store";
const BUFF_SIZE: usize = 1024;

//TODO change everything in this file from i32 to i64
pub fn open_kv_file() -> File {
    let file = match File::open(KV_FILE) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", KV_FILE,
                           Error::description(&why)),
        Ok(file) => file,
    };
    return file;
}

/*
pub fn seek_end(file: &Result<File>) {
    let f = try!(file);
    try!(f.seek(SeekFrom::End(0)));
}
 */

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

//TODO doesn't work. Hurray
pub fn read_key(file: &mut File, key_size: i32) -> String {
    //TODO this is limited to only read data that fits in size. Improve
    let mut key_buffer = [0; BUFF_SIZE];
    let mut file_take = file.take(key_size as u64);
    match file_take.read(&mut key_buffer) {
        Err(why) => panic!("Could not read size bytes. Err:{}",why),
        Ok(x) => {
            println!("File read:{}", x)
        }
    }
    let res = String::from_utf8_lossy(&mut key_buffer).into_owned();
    return res
}
