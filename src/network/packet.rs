use serde::{Serialize, Deserialize, Deserializer};
pub const MAX_CHUNK_SIZE: usize = 1024;

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Header {
        chunk_count: usize,
        total_size: usize,
        filename: String,
    },
    Content{
        index: usize,
        bytes: Vec<u8>,
    },
}
