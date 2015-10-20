use std::string::String;
use std::fs::File;
use std::error::Error;
use std::io::SeekFrom;
use std::io::prelude::*;

//TODO do this in a smart way
const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v6_store";

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

pub fn read_key(file: &mut File, key_size: i32) -> String {
    //let mut key_buffer = [0; (key_size+1) as usize];
    let mut key_buffer = Vec::with_capacity( (key_size + 1) as usize);
    let mut key_box = key_buffer.into_boxed_slice();
    //let mut key_slice = &mut key_buffer;
    //key_slice + 1;
    println!("Key buffer:{:?}", key_box);
    match file.read(&mut key_box) {
        Err(why) => panic!("Could not read size bytes. Err:{}",why),
        Ok(x) => {
            println!("File read:{}", x);
        }
    }
    key_buffer = key_box.into_vec();
    let mut res = match String::from_utf8(key_buffer) {
        Err(why) => panic!("Could not parse key string from bytes read. Err:{}", why),
        Ok(str_parsed) => str_parsed,
    };
    return res
}
