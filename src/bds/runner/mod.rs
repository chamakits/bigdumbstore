//use super::mode;
use super::file;

//TODO change all the seek/read stuff to just one method each.

pub fn reading(read_args: Vec<String>) {
    let mut file_opened = file::open_kv_file();
    let file_mut = &mut file_opened;
    file::seek_end(file_mut);
    let mut pos = file::seek_key_size(file_mut);

    if pos == 0 {
        //TODO print to error
        println!("Error! It seems this file is malformed, and only contains size for a first key");
    }
    let mut key_size = file::read_size(file_mut);
    println!("Key size:{}", key_size);

    file::seek_value_size_post_read_key_size(file_mut);
    let mut value_size = file::read_size(file_mut);
    println!("Value size:{}", value_size);

    file::seek_key(file_mut, key_size);
    let mut key = file::read_key(file_mut, key_size);
    println!("Key: {}", key);
}
