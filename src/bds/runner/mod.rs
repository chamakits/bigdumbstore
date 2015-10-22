//use super::mode;
use super::file;

//TODO change all the seek/read stuff to just one method each.
//TODO have this return a value instead of printing out
pub fn reading(read_args: Vec<String>) {
    let mut file_opened = file::open_kv_file();
    let file_mut = &mut file_opened;
    file::seek_end(file_mut);
    let mut is_key_found = false;
    while !is_key_found {

        println!("About to seek key_size");
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
        let key = file::read_key(file_mut, key_size);
        println!("Key: {}", key);

        let key_to_find_src = read_args.get(0).unwrap();

        let key_to_find = key_to_find_src;
        let key_check = &key;
        is_key_found = key_to_find == key_check;

        println!("Comparing {} == {}?: {}", key_to_find, key_check, is_key_found);
        println!("To find bytes: {:?} ; key bytes: {:?}", key_to_find.to_string().into_bytes(), key_check.to_string().into_bytes());
        /*
        key_to_find+1;
        (*key_to_find)+1;
        key+1;
         */

        let position_of_next_key = file::seek_value(file_mut, value_size, key_size);
        println!("Seeked");

        if is_key_found {
            let mut value_found = file::read_key(file_mut, value_size);
            println!("Value found:{}", value_found);
        } else if position_of_next_key == 0 {
            return;
        }
    }
    
}
