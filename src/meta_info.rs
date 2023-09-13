use bendy::{decoding::{Error, FromBencode, Object, ResultExt}};
use serde::{Deserialize, Serialize};

use crate::info::Info;

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaInfo {
    pub announce: Option<String>,                                // Required fields for all torrents
    pub info: Info,                                              // Required fields for all torrents

    pub announce_list:          Option<Vec<String>>,             // Optional
    pub comment:                Option<String>,                  // Optional
    pub created_by:             Option<String>,                  // Optional
    pub creation_date:          Option<u64>,                     // Optional
    pub encoding:               Option<String>,                  // Optional
    pub http_seeds:             Option<Vec<String>>,             // Optional
    pub other_fields:           Option<Vec<(String, String)>>,   // Optional
}

impl FromBencode for MetaInfo {

    const EXPECTED_RECURSION_DEPTH: usize = Info::EXPECTED_RECURSION_DEPTH + 15;

    fn decode_bencode_object(object: Object) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut announce:       Option<String>                = None;               // Required fields for all torrents
        let mut info:           Option<Info>                  = None;               // Required fields for all torrents

        let mut announce_list:  Option<Vec<String>>           = None;               // Optional Field
        let mut comment:        Option<String>                = None;               // Optional Field
        let mut created_by:     Option<String>                = None;               // Optional Field
        let mut creation_date:  Option<u64>                   = None;               // Optional Field
        let mut encoding:       Option<String>                = None;               // Optional Field
        let mut http_seeds:     Option<Vec<String>>           = None;               // Optional Field
        let mut other_fields:   Option<Vec<(String, String)>> = Some(Vec::new());   // Initialize other_fields as Some(Vec::new())

        let mut dict_dec: bendy::decoding::DictDecoder<'_, '_> = object.try_into_dictionary()?;

        while let Some(pair) = dict_dec.next_pair()? {
            match pair {
                (b"announce", value)        => announce         = String::decode_bencode_object(value).context("announce").map(Some)?,
                (b"announce_list", value)   => announce_list    = Vec::decode_bencode_object(value).context("announce_list").map(Some)?,
                (b"comment", value)         => comment          = String::decode_bencode_object(value).context("comment").map(Some)?,
                (b"created by", value)      => created_by       = String::decode_bencode_object(value).context("created by").map(Some)?,
                (b"creation date", value)   => creation_date    = u64::decode_bencode_object(value).context("creation_date").map(Some)?,
                (b"encoding", value)        => encoding         = String::decode_bencode_object(value).context("encoding").map(Some)?,
                (b"http seeds", value)      => http_seeds       = Vec::decode_bencode_object(value).context("http_seeds").map(Some)?,
                (b"info", value)            => info             = Info::decode_bencode_object(value).context("info").map(Some)?,

                (unknown_field, value) => {
                    let field_name: String = String::from_utf8_lossy(unknown_field).to_string();
                    let value_as_string: String;

                    match value {
                        Object::Bytes(bytes) => {
                            value_as_string = String::decode_bencode_object(bendy::decoding::Object::Bytes(bytes)).unwrap_or_else(|_| "Unknown Bytes".to_string());
                        }
                        Object::Integer(integer) => {
                            value_as_string = u64::decode_bencode_object(bendy::decoding::Object::Integer(integer)).map(|u| u.to_string()).unwrap_or_else(|_| "Unknown Integer".to_string());
                        }
                        Object::List(list) => {
                            value_as_string = Vec::<String>::decode_bencode_object(Object::List(list)).map(|v: Vec<String>| v.join(", ")).unwrap_or_else(|_| "Unknown List".to_string());
                        }
                        Object::Dict(_) => {
                            // Handle the case where the value is a dictionary if needed
                            value_as_string = "Dictionary".to_string();
                        }
                    }
               
                    let tuple: (String, String) = (field_name, value_as_string);
                    if let Some(ref mut fields) = other_fields {
                        fields.push(tuple);
                    }
                }
            }
        }
        
        let info: Info = info.ok_or_else(|| Error::missing_field("info"))?;
        Ok(MetaInfo {
            announce,
            info,
            announce_list,
            comment,
            created_by,
            creation_date,
            encoding,
            http_seeds,
            other_fields,
        })
    }

}
