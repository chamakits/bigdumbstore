use super::file;
use std::io;

pub fn reading(read_args: Vec<String>) {
    debug!("Will be reading with args: {:?}", read_args);
    let key_to_find = read_args.get(0).unwrap();

    let mut file_opened = file::open_kv_file_read();
    let file_mut = &mut file_opened;
    let mut bds = file::BdsFile::new(file_mut);

    let value_found = bds.find_value_by_key(key_to_find);
    println!("{}", value_found.unwrap());
}

pub fn writing(write_args: Vec<String>) {
    let key_to_write = write_args.get(0).unwrap();
    let mut stdin = &mut io::stdin();

    let mut file_opened = file::open_kv_file_write();
    let file_mut = &mut file_opened;

    let mut bds = file::BdsFile::new(file_mut);
    bds.write_to_key_from_stdin(key_to_write, stdin);
}
