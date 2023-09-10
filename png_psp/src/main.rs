#![feature(macro_metavar_expr)]
#![no_std]
#![no_main]

use core::ffi::c_void;
pub struct Args {
  count: usize,
  values: *mut u8
}

impl Args {
  fn new(argc: usize, argv: *mut c_void) -> Args {
    Self {
      count:argc,
      values:argv as *mut u8
    }
  }
  pub fn at(&self, i: usize) -> Option<u8> {
    if i > self.count {
      None
    } else {
      unsafe {
        Some(*self.values.add(i))
      }
    } 
  }
}

extern crate alloc;

pub use self::stb_image::*;
pub use self::stb_image_common::*;

mod c_runtime;
mod psp_module;
mod stb_image;
mod stb_image_common;
mod stb_image_png;
mod stb_image_zlib;

#[no_mangle]
pub extern "C" fn stbi_load(
  buffer: *const u8,
  len: i32,
  x: *mut i32,
  y: *mut i32,
  comp: *mut i32,
  req_comp: i32,
) -> *mut u8 {
  unsafe {
    stbi_load_from_memory(buffer, len, x, y, comp, req_comp)
  }
}


