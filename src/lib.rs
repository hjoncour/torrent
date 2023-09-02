pub mod meta_info;
pub mod info;
pub mod encode;
pub mod decode;

use bendy::decoding::{Error, FromBencode};
use crate::meta_info::MetaInfo;

pub fn open_torrent(input: &[u8]) -> Result<MetaInfo, Error> {
    return MetaInfo::from_bencode(&input);
}