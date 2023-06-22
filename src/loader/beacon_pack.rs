// BeaconPack rust port from https://github.com/trustedsec/COFFLoader/blob/main/beacon_generate.py
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;

pub struct BeaconPack {
    pub buffer: Vec<u8>,
    pub size: u32,
}

/// BeaconPack is a struct that contains a buffer and size
/// The buffer is used to store the data that will be sent to the BOF's arguments
impl BeaconPack {
    /// new returns a new BeaconPack
    pub fn new() -> BeaconPack {
        BeaconPack {
            buffer: vec![],
            size: 0,
        }
    }

    /// get_buffer returns the buffer with the size prepended
    pub fn get_buffer(&self) -> Vec<u8> {
        let mut result = vec![];
        result.write_u32::<LittleEndian>(self.size).unwrap();
        result.extend(&self.buffer);
        result
    }

    /// add_short adds a short to the buffer
    pub fn add_short(&mut self, short: i16) {
        self.buffer.write_i16::<LittleEndian>(short).unwrap();
        self.size += 2;
    }

    /// add_int adds an int to the buffer
    pub fn add_int(&mut self, int: i32) {
        self.buffer.write_i32::<LittleEndian>(int).unwrap();
        self.size += 4;
    }

    /// add_str adds a string to the buffer
    pub fn add_str(&mut self, str: &str) {
        let s_bytes = str.as_bytes();
        self.buffer
            .write_u32::<LittleEndian>((s_bytes.len() + 1) as u32)
            .unwrap();
        self.buffer.write_all(s_bytes).unwrap();
        self.buffer.write_u8(0).unwrap();
        self.size += (s_bytes.len() + 1) as u32 + 4;
    }

    /// add_wstr adds a wide string to the buffer
    pub fn add_wstr(&mut self, wstr: &str) {
        let s_bytes = wstr.encode_utf16().collect::<Vec<u16>>();
        self.buffer
            .write_u32::<LittleEndian>(((s_bytes.len() * 2) + 2) as u32)
            .unwrap();
        for c in &s_bytes {
            self.buffer.write_u16::<LittleEndian>(*c).unwrap();
        }
        self.buffer.write_u16::<LittleEndian>(0).unwrap();
        self.size += ((s_bytes.len() * 2) + 2) as u32 + 4;
    }
}
