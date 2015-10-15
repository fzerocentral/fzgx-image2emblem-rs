extern crate byteorder;
use self::byteorder::{ByteOrder, BigEndian};

extern crate image;
use self::image::GenericImage;

extern crate itertools;
use self::itertools::Itertools;

use checksum::checksum;

macro_rules! byte {
    ( $i:ident, $v:ident << $elem:expr ) => {
        $v[$i] = $elem;
        $i += 1;
    }
}

macro_rules! bytes {
    ( $i:ident , $v:ident << $ary:expr ) => {
        for byte in $ary.iter() {
            $v[$i] = *byte;
            $i += 1;
        }
    }
}

macro_rules! push {
    ( $v:ident << $ary:expr ) => {
        for byte in $ary.iter() {
            $v.push(byte);
        }
    }
}


fn gametitle() -> [u8; 32] {
    let mut gametitle: [u8; 32] = [0x00; 32];
    let fzgx = "F-Zero GX".as_bytes();

    for i in 0..fzgx.len() {
        gametitle[i] = fzgx[i];
    }

    gametitle
}

fn read_block(emblem_data: &mut Vec<u8>,
              image: &image::DynamicImage,
              alpha_threshold: i8, i: u32, j: u32) {
    let i = i as u32;
    let j = j as u32;

    for x in i..i+4 {
        for y in j..j+4 {
            let pixel = image.get_pixel(x, y).data;
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let a = pixel[3];

            match a < alpha_threshold as u8 {
                true => { emblem_data.push(0x00); },
                false => {
                    let red   = (r / 8) as u16;
                    let green = (g / 8) as u16;
                    let blue  = (b / 8) as u16;
                    let alpha: u16 = 1;
                    let value: u16 = 32768*alpha + 1024*red + 32*green + blue;

                    let mut buf: [u8; 2] = [0x00; 2];
                    byteorder::BigEndian::write_u16(&mut buf, value as u16);

                    for byte in buf.iter() {
                        emblem_data.push(*byte);
                    }
                }
            }
        }
    }
}



const FZGX: [u8; 4] = *b"GFZE";
const SEGA: [u8; 2] = *b"8P";


pub struct Emblem {
  gamecode:     [u8; 4],  // GFZE
  company:      [u8; 2],  // 8P
  reserved01:    u8,      // 0xFF
  banner_fmt:    u8,      // 0x02
  filename:     [u8; 32],
  timestamp:    [u8; 4],
  icon_addr:    [u8; 4], // 0x00 0x00 0x00 0x60
  icon_fmt:     [u8; 2], // 0x00 0x02
  icon_speed:   [u8; 2], // 0x00 0x03
  permission:    u8,
  copy_counter:  u8,
  index:        [u8; 2],
  filesize8:    [u8; 2], // 0x00 0x03
  reserved02:   [u8; 2], // 0xFF 0xFF
  comment_addr: [u8; 4], // 0x00 0x00 0x00 0x04

  pub checksum:     [u8; 2],
  something3:   [u8; 2], // 0x04 0x01
  game_title:   [u8; 32], // "F-ZERO GX" 0x00...
  file_comment: [u8; 60], // "YY/MM/DD HH:MM" 0x00...
  pub banner_data:  [u8; 6144], // banner pixel data (92 x 32 px)
  icon_data:    [u8; 2048], // icon pixel data (64 x 64 px)
  emblem_data:  [u8; 8192], // emblem pixel data (64 x 64 px)
  padding:      [u8; 8096] // 0x00 padding
}

impl Default for Emblem {
    fn default() -> Self {
        let game_title = gametitle();

        Emblem {
          gamecode:      FZGX,
          company:       SEGA,
          reserved01:    0xFF,
          banner_fmt:    0x02,
          filename:     [0x00; 32],
          timestamp:    [0x00; 4],
          icon_addr:    [0x00, 0x00, 0x00, 0x60],
          icon_fmt:     [0x00, 0x02],
          icon_speed:   [0x00, 0x03],
          permission:    0x04,
          copy_counter:  0x00,
          index:        [0x00, 0x00],
          filesize8:    [0x00, 0x03],
          reserved02:   [0xFF, 0xFF],
          comment_addr: [0x00, 0x00, 0x00, 0x04],

          checksum:     [0x00; 2],
          something3:   [0x04, 0x01],
          game_title:   game_title, // "F-ZERO GX" 0x00...
          file_comment: [0x00; 60], // "YY/MM/DD HH:MM" 0x00...

          banner_data:  [0x00; 6144], // banner pixel data (92 x 32 px)
          icon_data:    [0x00; 2048], // icon pixel data (64 x 64 px)
          emblem_data:  [0x00; 8192], // emblem pixel data (64 x 64 px)
          padding:      [0x00; 8096] // 0x00 padding
        }
    }
}

fn make_bytes(initial: &mut [u8], bytes: &[u8]) {
    for i in 0..bytes.len() {
        initial[i] = bytes[i];
    }
}

impl Emblem {
    pub fn set_filename(self: &mut Self, filename: String) {
        make_bytes(&mut self.filename, &filename.as_bytes());
    }

    pub fn set_timestamp(self: &mut Self, time: u32) {
        let mut buf = [0x00; 4];
        byteorder::BigEndian::write_u32(&mut buf, time);

        self.timestamp = buf;
    }

    pub fn set_comment(self: &mut Self, comment: String) {
        make_bytes(&mut self.file_comment, &comment.as_bytes());
    }

    pub fn set_emblem_data(self: &mut Self, image: image::DynamicImage, alpha_threshold: i8) {
        let mut v = Vec::new();

        for block_row in (0..image.width()).step(4) {
            for block_col in (0..image.width()).step(4) {
                read_block(&mut v, &image, alpha_threshold, block_row, block_col);
            }
        }

        for i in 0..v.len() {
            self.emblem_data[i] = v[i];
        }
    }

    pub fn set_icon_data(self: &mut Self) {
        let icon = include_bytes!("../data/emblem_icon");

        for i in 0..icon.len() {
            self.icon_data[i] = icon[i];
        }
    }

    pub fn set_banner_data(self: &mut Self, image: image::DynamicImage, alpha_threshold: i8) {
        let mut v: Vec<u8> = Vec::new();
        let banner_file = include_bytes!("../data/emblem_banner_base");
        let mut chunked_banner = banner_file.chunks(0x200);

        for block_row in (0..32).step(4) {
            for chunk in chunked_banner.nth(0) {
                for byte in chunk {
                    v.push(*byte);
                }
            }

            for block_col in (0..32).step(4) {
                read_block(&mut v, &image, alpha_threshold, block_row, block_col);
            }
        }

        for i in 0..v.len() {
            self.banner_data[i] = v[i];
        }
    }

    pub fn set_checksum(self: &mut Self) {
        let mut v = Vec::new();

        push!(v << self.something3);
        push!(v << self.game_title);
        push!(v << self.file_comment);
        push!(v << self.banner_data);
        push!(v << self.icon_data);
        push!(v << self.emblem_data);
        push!(v << self.padding);

        self.checksum = checksum(v);
    }

    pub fn as_bytes(self: Self) -> [u8; 24640] {
        let mut v: [u8; 24640] = [0x00; 24640];
        let mut index = 0;

        bytes!(index, v << self.gamecode);
        bytes!(index, v << self.company);
        byte!(index, v << self.reserved01);
        byte!(index, v << self.banner_fmt);
        bytes!(index, v << self.filename);
        bytes!(index, v << self.timestamp);
        bytes!(index, v << self.icon_addr);
        bytes!(index, v << self.icon_fmt);
        bytes!(index, v << self.icon_speed);
        byte!(index, v << self.permission);
        byte!(index, v << self.copy_counter);
        bytes!(index, v << self.index);
        bytes!(index, v << self.filesize8);
        bytes!(index, v << self.reserved02);
        bytes!(index, v << self.comment_addr);
        bytes!(index, v << self.checksum);
        bytes!(index, v << self.something3);
        bytes!(index, v << self.game_title);
        bytes!(index, v << self.file_comment);
        bytes!(index, v << self.banner_data);
        bytes!(index, v << self.icon_data);
        bytes!(index, v << self.emblem_data);
        bytes!(index, v << self.padding);

        v
    }
}
