use std::io::{Read, self};

pub enum Key {
    Esc,
    Digit(u8),
    Invalid,
}

pub fn read_key() -> io::Result<Key> {
    let mut buf = [0u8; 1];
    io::stdin().read_exact(&mut buf)?;

    match buf[0] {
        0x1B => Ok(Key::Esc),
        b'0'..=b'9' => Ok(Key::Digit(buf[0] - b'0')),
        _ => Ok(Key::Invalid),
    }
}
