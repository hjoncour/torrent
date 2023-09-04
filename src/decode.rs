use std::env;
use std::fs::File;
use std::io::Read;
use serde_json;
use crate::meta_info::MetaInfo;

pub fn open_file(filename: &str) -> MetaInfo {
    let local = env::current_dir().unwrap();
    let path = local.join("encTorrents/").join(filename);
    let mut file = File::open(path).expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Failed to read file");
    serde_json::from_str(&json_data).expect("JSON parsing error")
}
