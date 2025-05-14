use serde::{Serialize, Deserialize, Deserializer};

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Header {
        chunk_count: usize,
        total_size: usize,
        filename: String,
    },
    Content(Vec<u8>),
}
