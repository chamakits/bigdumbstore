use std::string::String;
use std::fs::File;
use std::error::Error;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::fs::OpenOptions;
use bit_vec::BitVec;

// TODO change everything in this file from i32 to i64

// TODO do this in a smart way
const BUFF_SIZE: usize = 1024;
const VALUE_ENTRY_MAX_SIZE: usize = 999;

#[derive(Debug)]
pub struct MetaData {
    pub is_final: bool,
}

impl MetaData {
    pub fn new(bit_vec: BitVec) -> MetaData {
        MetaData {
            is_final:  bit_vec.get(7).unwrap(),
        }
    }
    pub fn new_final() -> MetaData {
        MetaData {
            is_final:  true,
        }
    }
    pub fn new_not_final() -> MetaData {
        MetaData {
            is_final:  false,
        }
    }

    pub fn to_bit_vec(&self) -> BitVec {
        let mut bit_vec = BitVec::from_elem(8, false);
        bit_vec.set(7, self.is_final);
        debug!("to_bit_vec: {:?}", bit_vec);
        bit_vec
    }

    pub fn write_format(&self) -> String {
        match String::from_utf8(self.to_bit_vec().to_bytes()) {
            Ok(string) => string,
            Err(why) => panic!("Could not get string. Err: [{}]", why),
        }
    }
}

#[derive(Debug)]
pub struct BdsFile {
    bds_file: File,
}
const SEEK_GOTO_END: SeekFrom = SeekFrom::End(0);
const SEEK_META_DATA: SeekFrom = SeekFrom::Current(-1);
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

    pub fn find_value_by_key_from_beginning(&mut self, key_to_find: &str) -> Option<String> {
        let file_mut = &mut self.bds_file;
        BdsFile::seek_start_of_file_fail_if_empty(file_mut, key_to_find);
        BdsFile::find_value_by_key(file_mut, key_to_find)
    }

    //static mut lookup_count: u64 = 0;
    fn find_value_by_key(file_mut: &mut File, key_to_find: &str) -> Option<String> {
        //let file_mut = &mut self.bds_file;
        let mut is_key_found = false;
        let mut option_val: Option<String> = Option::None;
        while !is_key_found {
            let metadata = BdsFile::read_metadata(file_mut);
            debug!("Metadata found: {:?}", metadata);
            BdsFile::seek_with(file_mut, SEEK_META_DATA);
            let key_size = BdsFile::read_key_size(file_mut);
            let value_size = BdsFile::read_value_size(file_mut);

            //TODO: Look into whether this comparison is worth it or not.
            if key_size != key_to_find.len() as i64 {
                BdsFile::seek_value(file_mut, value_size, 3+key_size);

                continue;
            }
            let key_to_check = &BdsFile::read_key_string(file_mut, key_size);

            is_key_found = key_to_find == key_to_check;

            debug!("Comparing {} == {}?: {}",
                   key_to_find,
                   key_to_check,
                   is_key_found);

            let position_of_next_key = BdsFile::seek_value(file_mut, value_size, key_size);

            if is_key_found {
                match metadata.is_final {
                    true =>
                        option_val = BdsFile::read_value_string_option(file_mut, value_size),
                    false => {
                        option_val = BdsFile::read_value_string_option(file_mut, value_size);
                        let pos = BdsFile::seek_with(
                            file_mut,
                            SeekFrom::Current(-(((value_size) as usize) as i64)));
                        if pos == 0 {
                            panic!("Malformed file.  Marked value as unterminated, but reached file end for key to find: [{}]",
                                   key_to_find);
                        }
                        //TODO currently doing recursion, not ideal, think of changing.
                        debug!("Skipping full value as it is not terminated, after reading: {}", pos);
                        option_val = BdsFile::concat_option_strings(
                            BdsFile::find_value_by_key(file_mut, key_to_find),
                            option_val);
                    },
                }
                break;
            } else if position_of_next_key == 0 {
                option_val = Option::None;
                break;
            }
        }
        option_val
    }

    fn concat_option_strings(left: Option<String>, right: Option<String>) -> Option<String> {
        let left_string = match left {
            Some(string) => string,
            None => "".to_string(),
        };

        let right_string = match right {
            Some(string) => string,
            None => "".to_string(),
        };

        Option::Some(format!("{}{}", left_string, right_string))
    }
    
    fn seek_start_of_file_fail_if_empty(file_mut: &mut File, key_to_find: &str) {
        let pos = BdsFile::seek_with(file_mut, SEEK_GOTO_END);
        debug!("Position after reading to end: {}", pos);
        if pos == 0 {
            panic!("File found, but is empty. Cannot look for: [{}]",
                   key_to_find);
        }
    }

    fn read_metadata(file_mut: &mut File) -> MetaData {
        BdsFile::seek_metadata(file_mut);
        let mut metadata_buffer = [0; 1];
        debug!("metadata_buffer: {:?}", metadata_buffer);
        BdsFile::read_metadata_into_bytes(file_mut, &mut metadata_buffer);
        let bit_vec = BitVec::from_bytes(&metadata_buffer);
        MetaData::new(bit_vec)
    }

    fn seek_metadata(file_mut: &mut File) {
        let pos = BdsFile::seek_with(file_mut, SEEK_META_DATA);
        if pos == 0 {
            error!("Error! It seems this file is malformed, and only metadata for a first key");
        }
    }

    fn read_metadata_into_bytes(file: &mut File, buffer_to_read: &mut [u8]) {
        match file.read(buffer_to_read) {
            Err(why) => panic!("Could not read size bytes. Err:{}", why),
            _ => {}
        }
    }

    fn read_key_size(file_mut: &mut File) -> i64 {
        debug!("About to seek key_size");
        let pos = BdsFile::seek_with(file_mut, SEEK_KEY_SIZE);
        debug!("Position after seeking: {}", pos);
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
        let value_found = BdsFile::read_key(file_mut, value_size);
        debug!("Value found:'{}'", value_found);
        let option_val = Option::Some(value_found);
        option_val
    }

    pub fn write_to_key_from_stdin(&mut self, key: &str, stdin: &mut Read) {
        let mut string_in = &mut String::with_capacity(BUFF_SIZE);
        let _stdin_read_size = stdin.read_to_string(string_in);
        debug!("_stdin_read_size:{:?}, string_in.len:{}, BUFF_SIZE:{}",
               _stdin_read_size, string_in.len(), BUFF_SIZE);
        
        let stdin_read_size = _stdin_read_size.unwrap();
        string_in.truncate(stdin_read_size);

        self.write_to_key(key, string_in, stdin_read_size, MetaData::new_final());
    }

    fn write_to_key(&mut self, key: &str, string_in: &str,
                    stdin_read_size: usize, metadata: MetaData) {
        if stdin_read_size > VALUE_ENTRY_MAX_SIZE {
            let split_val = string_in.split_at(VALUE_ENTRY_MAX_SIZE);

            let first_half = split_val.0;
            let second_half = split_val.1;
            
            self.write_key_value(key, first_half, metadata, first_half.len());
            //TODO using recursion. Look into doing this without recursion.
            self.write_to_key(key, second_half, second_half.len(),
                              MetaData::new_not_final());
        } else {
            debug!("Read from input:{}", string_in);
            self.write_key_value(key, string_in, metadata, stdin_read_size);   
        }
    }

    pub fn write_to_key_dynamic(&mut self, key: &str, value_in: &str) {
        self.write_to_key(key, value_in, value_in.len(), MetaData::new_final())
    }

    fn write_key_value(&mut self, key: &str, value: &str,
                       metadata: MetaData, stdin_read_size: usize) {
        debug!("Will get string to write.");
        let to_write = format!(
            "{}{}{:03}{:03}{}",
            value, key, stdin_read_size, key.len(), metadata.write_format());
        debug!("Will write:{}.", to_write);

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
        let mut size_str = String::from_utf8_lossy(&mut size_buffer);
        debug!("Will try to convert size found: {}", size_str);
        let res = match size_str.to_mut().parse::<i64>() {
            Err(why) => panic!("Could not convert size to read to int [{}]. Err:{}", size_str, why),
            Ok(size_read) => size_read,
        };
        return res;
    }

    fn seek_key(file: &mut File, key_size: i64) -> u64 {
        BdsFile::seek_with(file, SeekFrom::Current(-(key_size + 3)))
    }

    fn read_key(file: &mut File, key_size: i64) -> String {
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



#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn mock_forcing_functions_compiled() {
        #![allow(path_statements)]
        super::BdsFile::write_to_key_from_stdin;
    }
    
    //MetaData tests
    #[test]
    fn test_metadata_to_bit_vec() {
        let mut metadata = MetaData::new_final();
        assert_eq!(true, metadata.to_bit_vec().get(7).unwrap());
        metadata = MetaData::new_not_final();
        assert_eq!(false, metadata.to_bit_vec().get(7).unwrap());
    }

    fn string_from_u8(byte: u8) -> String {
        let vec_slice = vec![byte as u8];
        match String::from_utf8(vec_slice) {
            Ok(res) => res,
            Err(why) => panic!("Couldn't get string from vec_slice. Error: {}", why),
        }
    }
    
    #[test]
    fn test_metadata_write_format() {
        let metadata = MetaData::new_final();
        let formatted_vec_slice = string_from_u8(1 as u8);
        assert_eq!(formatted_vec_slice, metadata.write_format());

        let metadata = MetaData::new_not_final();
        let formatted_vec_slice = string_from_u8(0 as u8);
        assert_eq!(formatted_vec_slice, metadata.write_format());
    }

    use tempdir::TempDir;
    use std::fs;
    use super::super::tests::*;

    fn create_kv_file(tmp_path_str: &str) {
        match fs::File::create(tmp_path_str) {
            Err(why) => panic!("Couldn't create file. Err: {}, for path: {}",
                               why,
                               tmp_path_str),
            _ => {}
        }
    }
    
    #[test]
    fn test_new_write_dynamic_and_read() {
        let tmp_dir = TempDir::new("bds_kv_dir").unwrap();
        let tmp_path_str = &temp_file_path_string(&tmp_dir);
        {
            create_kv_file(tmp_path_str);
            let mut bds_file = BdsFile::new_write(tmp_path_str);
            bds_file.write_to_key_dynamic("given_key", "given_value");
        }

        {
            let mut bds_file = BdsFile::new_read(tmp_path_str);
            let found_val = bds_file.find_value_by_key_from_beginning("given_key");
            println!("Found val:{:?}", found_val);
            assert_eq!(Option::Some("given_value".to_string()), found_val);
        }
    }

    use std::io::Cursor;
    #[test]
    fn test_write_to_key_from_stdin() {
        super::super::super::setup_logging();
        info!("START: test_write_to_key_from_stdin");
        let _tmp_dir = TempDir::new("bds_kv_dir");
        let tmp_dir = _tmp_dir.unwrap();
        let tmp_path_str = &temp_file_path_string(&tmp_dir);
        create_kv_file(tmp_path_str);
        let mut bds_file = BdsFile::new_write(tmp_path_str);

        let string_as_stdin = "Given String as if stdin".to_string();
        let bytes = string_as_stdin.to_string().into_bytes();
        let mut cursor = Cursor::new(bytes);

        let given_key = "given-key";
        bds_file.write_to_key_from_stdin(given_key, &mut cursor);

        {
            let mut bds_file = BdsFile::new_read(tmp_path_str);
            let val_found = bds_file.find_value_by_key_from_beginning(given_key).unwrap();
            assert_eq!(string_as_stdin, val_found);
            //assert_eq!(Option::Some("given_value".to_string()), found_val);
        }

        info!("END: test_write_to_key_from_stdin");
    }

}
