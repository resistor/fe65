pub fn push_byte(mut bytes: Vec<u8>, b: u8) -> Vec<u8> {
    bytes.push(b);
    bytes
}

pub fn push_u16_le(mut bytes: Vec<u8>, val: u16) -> Vec<u8> {
    let le_bytes = val.to_le_bytes();
    bytes.extend(le_bytes);
    bytes
}
