use std::{env, process::exit};
use nid_generator::{nid, to_hex};

fn main() {
  let name = env::args().nth(1);
  let v = match name {
    None => { println!("must provide name of string"); exit(0);},
    Some(n) => n,
  };
  println!("0x{}",
    to_hex(
      nid(v.as_bytes())
        .to_be_bytes()));
}