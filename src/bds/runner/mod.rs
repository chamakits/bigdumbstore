use super::file;
use std::io;
use std::fs;
use std::path::Path;

//TODO do this in a smart way
//const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v6_store";
//const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v6_store_rust";
//const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v6_store_rust_struct";
//const KV_FILE: &'static str = "/home/chamakits/.config/big-dumb-store/.v7_store";
const KV_FILE: &'static str = "/home/chamakits/.config-2/big-dumb-store/.v7_store";

fn create_file_if_not_exist(kv_file_path_str: &str) {
    let kv_file_path = Path::new(kv_file_path_str);

    match fs::metadata(kv_file_path_str) {
        Err(_) => {
            create_directories_if_needed(kv_file_path.parent().unwrap());
            match fs::File::create(kv_file_path_str) {
                Err(why) => panic!("Couldn't create file. Err: {}", why),
                _ => {}
            }
        },
        _ => {}
    };
}

fn create_directories_if_needed(path: &Path) {
    match fs::create_dir_all(path) {
        Err(why) => panic!("Couldn't create directory path {:?}: error:{}", path, why),
        Ok(_) => {},
    };
}

pub fn reading(read_args: Vec<String>) {
    create_file_if_not_exist(KV_FILE);
    debug!("Will be reading with args: {:?}", read_args);
    let key_to_find = read_args.get(0).unwrap();

    let mut bds = file::BdsFile::new_read(KV_FILE);

    let value_found = bds.find_value_by_key(key_to_find);
    if value_found.is_some() {
        println!("{}", value_found.unwrap());
    }
}

pub fn writing(write_args: Vec<String>) {
    create_file_if_not_exist(KV_FILE);
    let key_to_write = write_args.get(0).unwrap();
    let mut stdin = &mut io::stdin();

    let mut bds = file::BdsFile::new_write(KV_FILE);
    bds.write_to_key_from_stdin(key_to_write, stdin);
}
