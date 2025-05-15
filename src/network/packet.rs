use serde::{Serialize, Deserialize, Deserializer};
pub const MAX_CHUNK_SIZE: usize = 1024;

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Header(Header),
    Content(Content),
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub chunk_count: usize,
    pub total_size: usize,
    pub filename: String,
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub index: usize,
    pub bytes: Vec<u8>,
}
