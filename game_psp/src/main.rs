#![no_std]
#![no_main]

mod io;
mod grafix;

use grafix::*;
use io::System;
use psp::sys::*;

psp::module!("sample_cube", 1, 1);

#[repr(C, align(4))]
struct Vertex {
  u: f32,
  v: f32,
  x: f32,
  y: f32,
  z: f32,
}


fn psp_main() {
  let _s = System::new();
  let _g = Graphics::new();
  let mut running = true;
  while running {
    let _f = Frame::new();

    unsafe {
      let color = create_color(0x00, 0xff, 0x00, 0xff);
      clear_color(color);
    }
  }
}

fn create_color(r: u8, g: u8, b: u8, a: u8) -> u32 {
  rgba(r, g, b, a)
}

fn clear_color(color: u32) {
  unsafe {
    sceGuClearColor(color);
    sceGuClear(
      ClearBuffer::COLOR_BUFFER_BIT | 
      ClearBuffer::DEPTH_BUFFER_BIT
    );
  }
}

