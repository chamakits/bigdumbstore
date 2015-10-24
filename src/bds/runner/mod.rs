use super::file;
use std::io;

//TODO do this in a smart way
//const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v6_store";
//const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v6_store_rust";
const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v6_store_rust_struct";


pub fn reading(read_args: Vec<String>) {
    debug!("Will be reading with args: {:?}", read_args);
    let key_to_find = read_args.get(0).unwrap();

    let mut bds = file::BdsFile::new_read(KV_FILE);

    let value_found = bds.find_value_by_key(key_to_find);
    if value_found.is_some() {
        println!("{}", value_found.unwrap());
    }
}

pub fn writing(write_args: Vec<String>) {
    let key_to_write = write_args.get(0).unwrap();
    let mut stdin = &mut io::stdin();

    let mut bds = file::BdsFile::new_write(KV_FILE);
    bds.write_to_key_from_stdin(key_to_write, stdin);
}
