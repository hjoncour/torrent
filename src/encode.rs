
use std::env;
use crate::meta_info::MetaInfo;

fn serialize_meta_info(meta_info: &MetaInfo) -> Result<String, serde_json::Error> {
    let json = serde_json::to_string(meta_info)?;
    Ok(json)
}

fn create_json(json: &str, name: &str) -> Result<(), std::io::Error> {
    let folder_path = env::current_dir()?.join("encTorrents");
    std::fs::create_dir_all(&folder_path)?;
    let file_path = folder_path.join(format!("{}.enctorrent", name));
    std::fs::write(file_path, json)?;
    Ok(())
}

pub fn save_to_json_file(input: MetaInfo) -> Result<String, Box<dyn std::error::Error>> {
    let name = &input.info.name;
    let json = serialize_meta_info(&input)?;
    create_json(&json, &name)?;
    Ok(name.to_string())
}