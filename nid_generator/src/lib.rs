#![no_std]

use core::hash::Hasher;
extern crate alloc;

use alloc::string::String;
use rs_sha1::Sha1Hasher;

pub fn nid(arg:&[u8]) -> u32 {
  le_truncated_sha1(arg)
    .into_iter()
    .map(|v| v as u32)
    .rev()
    .enumerate()
    .map(|(i,v)| v << i * 8)
    .fold(0, |prev, v| prev | v)
}

pub fn to_hex<const N:usize>(blob: [u8; N]) -> String {
  let mut buf = String::from("");
  
  for ch in blob {
    fn hex_from_digit(num: u8) -> char {
      if num < 10 { 
        (b'0' + num) as char 
      } else { 
        (b'a' + num - 10) as char 
      }
    }
    buf.push(hex_from_digit(ch / 16));
    buf.push(hex_from_digit(ch % 16));
  }
  buf
}

fn le_truncated_sha1(b:&[u8]) -> [u8; 4] {
  let mut result = truncated_sha1(b);
  result.reverse();
  result
}

fn truncated_sha1(a: &[u8]) -> [u8; 4] {
  let hash = sha1(a);
  truncate::<8,4>(hash)
}

fn sha1(s: &[u8]) -> [u8;8] {
  assert!(s.is_ascii());
  let mut sha1hasher = Sha1Hasher::default();

  sha1hasher.write(s);

  sha1hasher.finish().to_be_bytes()
}

fn truncate<const N:usize,const M:usize>(a: [u8; N]) -> [u8; M] {
  let mut result = [0u8; M];
  result.copy_from_slice(&a[..M]);
  result
}

#[cfg(test)]
mod test {
  use crate::*;
  
  const NAME: &[u8] = b"sceKernelLoadModule";

  #[test]
  fn it_is_nid() {
    assert_eq!(nid(NAME), 0x977de386);
  }
  
  #[test]
  fn it_hashes_a_known_hash() {
    assert_hex(sha1(NAME), "86e37d9746c65c61");
  }

  #[test]
  fn it_truncates_to_correct_length() {
    assert_hex(truncated_sha1(NAME), "86e37d97");
  }

  #[test]
  fn it_orders_bytes_in_little_endian() {
    assert_hex(le_truncated_sha1(NAME), "977de386");
  }

  fn assert_hex<const N: usize>(arg: [u8; N], arg2: &str) {
   assert_eq!(
      to_hex(arg),
      arg2,
    );
  }

  
  
}
