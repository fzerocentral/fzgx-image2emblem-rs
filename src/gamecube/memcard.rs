extern crate byteorder;
use self::byteorder::{BigEndian, ByteOrder};

use super::super::emblem::make_bytes;

pub enum Region {
    NTSC,
    PAL,
}

pub struct Memcard {
    pub gamecode: [u8; 4], // GFZE
    pub company: [u8; 2],  // 8P
    pub reserved01: u8,    // 0xFF
    pub banner_fmt: u8,    // 0x02
    pub filename: [u8; 32],
    pub timestamp: [u8; 4],
    pub icon_addr: [u8; 4],  // 0x00 0x00 0x00 0x60
    pub icon_fmt: [u8; 2],   // 0x00 0x02
    pub icon_speed: [u8; 2], // 0x00 0x03
    pub permission: u8,
    pub copy_counter: u8,
    pub index: [u8; 2],
    pub filesize8: [u8; 2],    // 0x00 0x03
    pub reserved02: [u8; 2],   // 0xFF 0xFF
    pub comment_addr: [u8; 4], // 0x00 0x00 0x00 0x04
}

impl Memcard {
    pub fn set_region(self: &mut Self, region: Region) {
        match region {
            Region::NTSC => self.gamecode = *b"GFZP",
            Region::PAL => self.gamecode = *b"GFZE",
        }
    }

    pub fn set_filename(self: &mut Self, filename: String) {
        make_bytes(&mut self.filename, &filename.as_bytes());
    }

    pub fn set_timestamp(self: &mut Self, time: u32) {
        let mut buf = [0x00; 4];
        byteorder::BigEndian::write_u32(&mut buf, time);

        self.timestamp = buf;
    }
}
