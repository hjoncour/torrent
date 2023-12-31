use std::path::{PathBuf};
use std::fs::{self, File};
use std::io::Write;
use serde_xml_rs;
use serde_yaml;
use crate::meta_info::MetaInfo;

enum FileType {
    Json,
    Xml,
    Yaml,
    Torrent,
}

impl FileType {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(FileType::Json),
            "xml" => Some(FileType::Xml),
            "yaml" | "yml" => Some(FileType::Yaml),
            "torrent" => Some(FileType::Torrent),
            _ => None,
        }
    }

    fn serialize(&self, input: &MetaInfo) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            FileType::Json => serialize_meta_info_json(input).map_err(|e| e.into()),
            FileType::Xml => serde_xml_rs::to_string(input).map_err(|e| e.into()),
            FileType::Yaml => serde_yaml::to_string(input).map_err(|e| e.into()),
            FileType::Torrent => Ok(String::new()),
        }
    }
}

fn serialize_meta_info_json(meta_info: &MetaInfo) -> Result<String, serde_json::Error> {
    let json = serde_json::to_string(meta_info)?;
    Ok(json)
}

pub fn save_to_json_file(input: MetaInfo, output_name: String, output_path: Option<&str>, file_type: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let output: String;
    let mut file: File;
    let mut file_path: PathBuf;
    
    match output_path {
        Some(n) => {
            file_path = PathBuf::from(n);
            file_path.push(output_name.clone());
        }
        None => {
            file_path = PathBuf::from(&output_name);
            file_path.push(output_name.clone());
        }
    }

    let chosen_file_type = match file_type.and_then(|ft| FileType::from_str(ft)) {
        Some(file_type_enum) => file_type_enum,
        None => FileType::Json,
    };

    output = chosen_file_type.serialize(&input)?;

    if let Some(dir) = file_path.parent() {
        fs::create_dir_all(dir)?;
    }

    file_path.set_extension(match chosen_file_type {
        FileType::Json => "json",
        FileType::Xml => "xml",
        FileType::Yaml => "yaml",
        FileType::Torrent => "torrent",
    });

    file = File::create(&file_path)?;
    file.write_all(output.as_bytes())?;
    
    Ok(output_name)
}
