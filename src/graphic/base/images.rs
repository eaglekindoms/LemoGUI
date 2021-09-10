#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ImageRaw {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}