#![no_std]
#![no_main]

mod io;
mod grafix;

use core::{ptr::null, ffi::c_void};

use grafix::*;
use io::System;
use psp::{sys::{*, self}, Align16};

psp::module!("sample", 1, 1);

#[repr(C, align(4))]
struct Vertex {
  color: u32,
  x: f32,
  y: f32,
  z: f32,
}

static TRIANGLE: Align16<[Vertex; 3]> = Align16([
  Vertex { color: rgba(0xFF,0x00,0xFF,0xFF), x: 0.35,  y:0.0,  z:-1.0 }, 
  Vertex { color: rgba(0x00,0xFF,0x00,0xFF), x:-0.35,  y:0.0,  z:-1.0 }, 
  Vertex { color: rgba(0xFF,0xFF,0xFF,0xFF), x: 0.0,   y:0.5,  z:-1.0 }
]);

fn psp_main() {
  let _s = System::new();
  let _g = Graphics::new();

  setup_matrices();

  loop {
    let _f = Frame::new();
  
    unsafe {
      sceGuDisable(GuState::DepthTest);
      sceGuDisable(GuState::Texture2D);

      let color = create_color(0x00, 0xA0, 0xA0, 0xff);
      clear_color(color);

      sceGumDrawArray(
        GuPrimitive::Triangles, 
        VertexType::COLOR_8888 | VertexType::VERTEX_32BITF |VertexType::TRANSFORM_3D, 
        3, 
        null(), 
        TRIANGLE.as_void_ptr()
      );
      
    }
  }
}

fn setup_matrices() {
  unsafe {
    sceGumMatrixMode(MatrixMode::Projection);
    sceGumLoadIdentity();
    sceGumOrtho(-16.0/9.0, 16.0/9.0, -1.0, 1.0, -10.0, 10.0);
    sceGumMatrixMode(MatrixMode::View);
    sceGumLoadIdentity();
    sceGumMatrixMode(MatrixMode::Model);
    sceGumLoadIdentity();
  }
}

trait ToVoid {
  fn as_void_ptr(&self) -> *const c_void;
}

impl<T> ToVoid for Align16<T> {
  fn as_void_ptr(&self) -> *const c_void {
    self as *const _ as *const c_void
  }
}

fn reset_translate(a:(f32,f32,f32)) {
  unsafe {
    sceGumMatrixMode(MatrixMode::Model);
    sceGumLoadIdentity();
    let v = ScePspFVector3{x: a.0, y: a.1, z:a.2};
    sceGumTranslate(&v);
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
      ClearBuffer::DEPTH_BUFFER_BIT |
      ClearBuffer::STENCIL_BUFFER_BIT
    );
  }
}

