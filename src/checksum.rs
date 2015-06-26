extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian};

pub fn checksum(bytes: Vec<&u8>) -> [u8; 2] {
  let mut checksum: u16 = 0xFFFF;
  let generator_polynomial = 0x8408;

  for byte in bytes {
    checksum = checksum ^ (*byte as u16);

    for _ in (0..8) {
      if checksum & 1 == 1 {
        checksum = (checksum >> 1) ^ generator_polynomial
      } else {
        checksum = checksum >> 1
      }
    }
  }

  checksum = checksum ^ 0xFFFF;
  let mut buf = [0u8; 2];
  byteorder::BigEndian::write_u16(&mut buf, checksum as u16);

  return buf;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn creates_correct_checksum() {
    let mut vec: Vec<&u8> = Vec::new();

    for byte in "FZGX".as_bytes() {
      vec.push(byte);
    }
    let checksum = checksum(vec);

    assert_eq!(checksum, [0x84, 0xC9]);
  }
}
