use bendy::decoding::{Error, FromBencode, Object, ResultExt};
use serde::{Deserialize, Serialize};

use crate::info::Info;

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaInfo {
    pub announce: String,
    pub info: Info,
    // Unofficial Elements
    pub comment: Option<String>,            // Optional
    pub creation_date: Option<u64>,         // Optional
    pub http_seeds: Option<Vec<String>>,    // Optional
}

impl FromBencode for MetaInfo {

    const EXPECTED_RECURSION_DEPTH: usize = Info::EXPECTED_RECURSION_DEPTH + 15;

    fn decode_bencode_object(object: Object) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut announce          = None;
        let mut comment           = None;
        let mut creation_date        = None;
        let mut http_seeds   = None;
        let mut info                = None;

        let mut dict_dec = object.try_into_dictionary()?;
        while let Some(pair) = dict_dec.next_pair()? {
            match pair {
                (b"announce", value)        => announce = String::decode_bencode_object(value).context("announce").map(Some)?,
                (b"comment", value)         => comment = String::decode_bencode_object(value).context("comment").map(Some)?,
                (b"creation date", value)   => creation_date = u64::decode_bencode_object(value).context("creation_date").map(Some)?,
                (b"httpseeds", value)       => http_seeds = Vec::decode_bencode_object(value).context("http_seeds").map(Some)?,
                (b"info", value)            => info = Info::decode_bencode_object(value).context("info").map(Some)?,
                //  Ignored
                (b"announce-list", _value)  => println!("announce-list: ignored"),
                (b"created by", _value)     => println!("created by: ignored"),
                (b"encoding", _value)       => println!("encoding: ignored"),
                (unknown_field, _)          => return Err(Error::unexpected_field(String::from_utf8_lossy(unknown_field)))
            }
        }

        let announce = announce.ok_or_else(|| Error::missing_field("announce"))?;
        let info = info.ok_or_else(|| Error::missing_field("info"))?;

        Ok(MetaInfo {
            announce,
            info,
            comment,
            creation_date,
            http_seeds,
        })
    }
}
