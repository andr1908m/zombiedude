#![no_std]
#![no_main]

mod io;
mod grafix;
mod c_compat;

use core::{ ptr::null, ffi::c_void };

use grafix::*;
use c_compat::ToVoid;
use io::System;
use psp::{ sys::{ *, self }, Align16 };

psp::module!("sample", 1, 1);

#[repr(C, align(4))]
struct Vertex {
  color: u32,
  x: f32,
  y: f32,
  z: f32,
}

static TRIANGLE: Align16<[Vertex; 3]> = Align16([
  Vertex { color: rgba(0xff, 0x00, 0xff, 0xff), x: 0.35, y: 0.0, z: -1.0 },
  Vertex { color: rgba(0x00, 0xff, 0x00, 0xff), x: -0.35, y: 0.0, z: -1.0 },
  Vertex { color: rgba(0xff, 0xff, 0xff, 0xff), x: 0.0, y: 0.5, z: -1.0 },
]);

static SQUARE: Align16<[Vertex; 6]> = Align16([
  Vertex { color: rgba(0xff, 0xff, 0x00, 0xff), x: -0.25, y: -0.25, z: -1.0 },
  Vertex { color: rgba(0xff, 0x00, 0xff, 0xff), x: -0.25, y: 0.25, z: -1.0 },
  Vertex { color: rgba(0xff, 0x00, 0x00, 0xff), x: 0.25, y: 0.25, z: -1.0 },

  Vertex { color: rgba(0xff, 0x00, 0x00, 0xff), x: 0.25, y: 0.25, z: -1.0 },
  Vertex { color: rgba(0xff, 0xff, 0x00, 0xff), x: 0.25, y: -0.25, z: -1.0 },
  Vertex { color: rgba(0xff, 0x00, 0xff, 0xff), x: -0.25, y: -0.25, z: -1.0 },
]);

static SQUARE_INDEXED: Align16<[Vertex; 4]> = Align16([
  Vertex { color: rgba(0xff, 0xff, 0x00, 0xff), x: -0.25, y: -0.25, z: -1.0 },
  Vertex { color: rgba(0xff, 0x00, 0xff, 0xff), x: -0.25, y: 0.25, z: -1.0 },
  Vertex { color: rgba(0xff, 0x00, 0x00, 0xff), x: 0.25, y: 0.25, z: -1.0 },
  Vertex { color: rgba(0xff, 0xff, 0x00, 0xff), x: 0.25, y: -0.25, z: -1.0 },
]);

static SQUARE_INDICES: Align16<[u16; 6]> = Align16([0, 1, 2, 2, 3, 0]);

fn psp_main() {
  let _s = System::new();
  let _g = Graphics::new();

  setup_matrices();

  loop {
    let _f = Frame::new();

    unsafe {
      sceGuDisable(GuState::DepthTest);
      sceGuDisable(GuState::Texture2D);

      let color = create_color(0x00, 0xa0, 0xa0, 0xff);
      clear_color(color);

      reset_translate((-0.5, 0.0, 0.0));
      draw_vertices(&TRIANGLE);

      reset_translate((0.5, 0.25, 0.0));
      draw_vertices(&SQUARE);

      reset_translate((0.5, -0.7, 0.0));
      draw_vertices_indexed(&SQUARE_INDICES, &SQUARE_INDEXED);
    }
  }
}

fn draw_vertices<const M:usize, T>(vertices: &Align16<[T;M]>) {
  unsafe {
    sceGumDrawArray(
      GuPrimitive::Triangles,
      VertexType::COLOR_8888 | 
        VertexType::VERTEX_32BITF | 
        VertexType::TRANSFORM_3D,
      vertices.0.len() as i32,
      null(),
      vertices.as_void_ptr()
    );
  }
}

fn draw_vertices_indexed<const N:usize, const M:usize, T>(
  indices: &Align16<[u16;N]>, vertices: &Align16<[T;M]>) {
  unsafe {
    sceGumDrawArray(
      GuPrimitive::Triangles,
      VertexType::INDEX_16BIT |
        VertexType::COLOR_8888 |
        VertexType::VERTEX_32BITF |
        VertexType::TRANSFORM_3D,
      vertices.0.len() as i32,
      indices.as_void_ptr(),
      vertices.as_void_ptr()
    )
  }
}

fn setup_matrices() {
  unsafe {
    sceGumMatrixMode(MatrixMode::Projection);
    sceGumLoadIdentity();
    sceGumOrtho(-16.0 / 9.0, 16.0 / 9.0, -1.0, 1.0, -10.0, 10.0);
    sceGumMatrixMode(MatrixMode::View);
    sceGumLoadIdentity();
    sceGumMatrixMode(MatrixMode::Model);
    sceGumLoadIdentity();
  }
}



fn reset_translate(a: (f32, f32, f32)) {
  unsafe {
    sceGumMatrixMode(MatrixMode::Model);
    sceGumLoadIdentity();
    let v = ScePspFVector3 { x: a.0, y: a.1, z: a.2 };
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
