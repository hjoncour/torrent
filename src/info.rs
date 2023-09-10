use bendy::decoding::{Error, FromBencode, Object, ResultExt};
use bendy::encoding::AsString;
use serde::{Deserialize, Serialize};

// File related information (Single-file format)
#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub piece_length: String,
    pub pieces: Vec<u8>,
    pub name: String,
    pub file_length: Option<String>,
    pub files: Option<Vec<String>>
}

impl FromBencode for Info {
    const EXPECTED_RECURSION_DEPTH: usize = 1;

    fn decode_bencode_object(object: Object) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut file_length:    Option<String>                  = None;
        let mut name:           Option<String>                  = None;
        let mut piece_length:   Option<String>                  = None;
        let mut pieces:         Option<Vec<u8>>                 = None;
        let mut other_fields:   Option<Vec<(String, String)>>   = Some(Vec::new()); // Initialize other_fields as Some(Vec::new())
        let mut files:          Option<Vec<String>>             = None;

        let mut dict_dec: bendy::decoding::DictDecoder<'_, '_> = object.try_into_dictionary()?;
        while let Some(pair) = dict_dec.next_pair()? {
            match pair {
                (b"length", value)          => file_length  = value.try_into_integer().context("file.length").map(ToString::to_string).map(Some)?,
                (b"name", value)            => name         = String::decode_bencode_object(value).context("name").map(Some)?,
                (b"piece length", value)    => piece_length = value.try_into_integer().context("length").map(ToString::to_string).map(Some)?,
                (b"pieces", value)          => pieces       = AsString::decode_bencode_object(value).context("pieces").map(|bytes| Some(bytes.0))?,
                // (b"files", value)           => files        = Vec::decode_bencode_object(value).context("files").map(Some)?,

                (unknown_field, value) => {
                    let field_name: String = String::from_utf8_lossy(unknown_field).to_string();
                    let value_as_string: String;

                    match &value {
                        Object::Bytes(bytes) => {
                            value_as_string = String::decode_bencode_object(bendy::decoding::Object::Bytes(bytes)).unwrap_or_else(|_| "Unknown".to_string());
                        }
                        Object::Integer(integer) => {
                            value_as_string = u64::decode_bencode_object(bendy::decoding::Object::Integer(integer)).map(|u| u.to_string()).unwrap_or_else(|_| "Unknown".to_string());
                        }
                        Object::List(list) => {
                            // Not working
                            // value_as_string = Vec::<String>::decode_bencode_object(Object::List(list)).map(|v: Vec<String>| v.join(", ")).unwrap_or_else(|_| "Unknown".to_string());
                            println!("missing list: __");//                            println!("list: {:#?}", list);
                            value_as_string = "".to_owned();
                        }
                        Object::Dict(_) => {
                            // TO FIX
                            println!("missing dict: __");//                            println!("list: {:#?}", list);
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

        let file_length: Option<String> = file_length.map(|s| s.to_string());
        let name:           String      = name.ok_or_else(|| Error::missing_field("name"))?;
        let piece_length:   String      = piece_length.ok_or_else(|| Error::missing_field("piece_length"))?;
        let pieces:         Vec<u8>     = pieces.ok_or_else(|| Error::missing_field("pieces"))?;

        // Check that we discovered all necessary fields
        Ok(Info {file_length, name, piece_length, pieces, files})
    }
}