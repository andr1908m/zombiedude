#![no_std]
#![no_main]

use core::ffi::c_void;
extern crate alloc;

pub use self::stb_image::*;
pub use self::stb_image_common::*;

use os_psp::*;
mod stb_image;
mod stb_image_common;
mod stb_image_png;
mod stb_image_zlib;

pub fn stbi_load(
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

fn library_call() {
  
}

// psp_export!("png_psp", (0, 1), stbi_load);

