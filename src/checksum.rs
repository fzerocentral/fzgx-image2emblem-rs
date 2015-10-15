extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian};

const GENERATOR_POLYNOMIAL: u16 = 0x8408;

fn checksum_bits(checksum: u16, bits_in_byte: i8) -> u16 {
  (0..bits_in_byte).fold(checksum, |acc, _| {
    match acc & 1 {
      1 => (acc >> 1) ^ GENERATOR_POLYNOMIAL,
      _ => acc >> 1
    }
  })
}

pub fn checksum(bytes: Vec<&u8>) -> [u8; 2] {
  let initial_mask: u16 = 0xFFFF;
  let bits_in_byte = 8;
  let checksum: u16 = bytes.iter().fold(initial_mask, |acc, &item| {
    checksum_bits(acc ^ (*item as u16), bits_in_byte)
  }) ^ initial_mask;

  let mut buf = [0u8; 2];
  byteorder::BigEndian::write_u16(&mut buf, checksum);

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
