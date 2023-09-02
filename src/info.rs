use bendy::decoding::{Error, FromBencode, Object, ResultExt};
use bendy::encoding::AsString;
use serde::{Deserialize, Serialize};

// File related information (Single-file format)
#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub piece_length: String,
    pub pieces: Vec<u8>,
    pub name: String,
    pub file_length: String,
}

impl FromBencode for Info {
    const EXPECTED_RECURSION_DEPTH: usize = 1;

    fn decode_bencode_object(object: Object) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut file_length = None;
        let mut name = None;
        let mut piece_length = None;
        let mut pieces = None;

        let mut dict_dec = object.try_into_dictionary()?;
        while let Some(pair) = dict_dec.next_pair()? {
            match pair {
                (b"length", value)          => file_length = value.try_into_integer().context("file.length").map(ToString::to_string).map(Some)?,
                (b"name", value)            => name = String::decode_bencode_object(value).context("name").map(Some)?,
                (b"piece length", value)    => piece_length = value.try_into_integer().context("length").map(ToString::to_string).map(Some)?,
                (b"pieces", value)          => pieces = AsString::decode_bencode_object(value).context("pieces").map(|bytes| Some(bytes.0))?,
                // Ignored
                (b"files", _value)          => println!("files: ignored"),
                (b"file_length", _value)    => println!("file_length: ignored"),

                (unknown_field, _) => return Err(Error::unexpected_field(String::from_utf8_lossy(unknown_field)))
            }
        }

        let file_length = file_length.ok_or_else(|| Error::missing_field("file_length"))?;
        let name = name.ok_or_else(|| Error::missing_field("name"))?;
        let piece_length = piece_length.ok_or_else(|| Error::missing_field("piece_length"))?;
        let pieces = pieces.ok_or_else(|| Error::missing_field("pieces"))?;

        // Check that we discovered all necessary fields
        Ok(Info {
            file_length,
            name,
            piece_length,
            pieces,
        })
    }
}