#![no_std]
#![no_main]

mod io;
mod grafix;
mod c_compat;
mod file;

extern crate alloc;

use core::{ ptr::{null, null_mut}, ffi::c_void, alloc::Layout, panic };

use alloc::{vec::{self, Vec}, alloc::{alloc, dealloc}};
use grafix::*;
use c_compat::ToVoid;
use io::System;
// use png_psp::{stbi_set_flip_vertically_on_load, STBI_rgb_alpha, stbi_load_from_memory, stbi_image_free};
use psp::{ sys::{ *, self }, Align16, vram_alloc };

use crate::file::File;

psp::module!("sample", 1, 1);

#[repr(C, align(4))]
struct Vertex {
  u: f32,
  v: f32,
  color: u32,
  x: f32,
  y: f32,
  z: f32,
}

static TRIANGLE: Align16<[Vertex; 3]> = Align16([
  Vertex { u: 0.0, v: 0.0, color: rgba(0xff, 0x00, 0xff, 0xff), x: 0.35, y: 0.0, z: -1.0 },
  Vertex { u: 0.0, v: 0.0, color: rgba(0x00, 0xff, 0x00, 0xff), x: -0.35, y: 0.0, z: -1.0 },
  Vertex { u: 0.0, v: 0.0, color: rgba(0xff, 0xff, 0xff, 0xff), x: 0.0, y: 0.5, z: -1.0 },
]);

static SQUARE: Align16<[Vertex; 6]> = Align16([
  Vertex { u: 0.0, v: 0.0, color: rgba(0xff, 0xff, 0x00, 0xff), x: -0.25, y: -0.25, z: -1.0 },
  Vertex { u: 0.0, v: 0.0, color: rgba(0xff, 0x00, 0xff, 0xff), x: -0.25, y: 0.25, z: -1.0 },
  Vertex { u: 0.0, v: 0.0, color: rgba(0xff, 0x00, 0x00, 0xff), x: 0.25, y: 0.25, z: -1.0 },

  Vertex { u: 0.0, v: 0.0, color: rgba(0xff, 0x00, 0x00, 0xff), x: 0.25, y: 0.25, z: -1.0 },
  Vertex { u: 0.0, v: 0.0, color: rgba(0xff, 0xff, 0x00, 0xff), x: 0.25, y: -0.25, z: -1.0 },
  Vertex { u: 0.0, v: 0.0, color: rgba(0xff, 0x00, 0xff, 0xff), x: -0.25, y: -0.25, z: -1.0 },
]);

static SQUARE_INDEXED: Align16<[Vertex; 4]> = Align16([
  Vertex { u: 0.0, v: 0.0, color: rgba(0xff, 0xff, 0x00, 0xff), x: -0.25, y: -0.25, z: -1.0 },
  Vertex { u: 0.0, v: 1.0, color: rgba(0xff, 0x00, 0xff, 0xff), x: -0.25, y: 0.25, z: -1.0 },
  Vertex { u: 1.0, v: 1.0, color: rgba(0xff, 0x00, 0x00, 0xff), x: 0.25, y: 0.25, z: -1.0 },
  Vertex { u: 1.0, v: 0.0, color: rgba(0xff, 0xff, 0x00, 0xff), x: 0.25, y: -0.25, z: -1.0 },
]);

static SQUARE_INDICES: Align16<[u16; 6]> = Align16([0, 1, 2, 2, 3, 0]);

fn psp_main() {
  
  let _s = System::new();
  let mut g = Graphics::new();

  setup_matrices();

  g.for_each_frame(|| {
    let color = create_color(0x00, 0xa0, 0xa0, 0xff);
    clear_color(color);

    reset_translate((-0.5, 0.0, 0.0));
    draw_vertices(&TRIANGLE);

    reset_translate((0.5, 0.25, 0.0));
    draw_vertices(&SQUARE);

    // let tex = load_texture("ms0:/./ferris.png");
    // bind_texture(&tex);
    // reset_translate((0.5, -0.7, 0.0));
    // draw_vertices_indexed(&SQUARE_INDICES, &SQUARE_INDEXED);
  });
}

fn draw_vertices<const M:usize, T>(vertices: &Align16<[T;M]>) {
  unsafe {
    sceGumDrawArray(
      GuPrimitive::Triangles,
      VertexType::TEXTURE_32BITF |
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
        VertexType::TEXTURE_32BITF | 
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
    sceGumLoadIdentity();
    sceGumMatrixMode(MatrixMode::Projection);
    sceGumOrtho(-16.0 / 9.0, 16.0 / 9.0, -1.0, 1.0, -10.0, 10.0);
    sceGumMatrixMode(MatrixMode::View);
    sceGumLoadIdentity();
    sceGumMatrixMode(MatrixMode::Model);
  }
}

fn reset_translate(a: (f32, f32, f32)) {
  unsafe {
    sceGumLoadIdentity();
    sceGumMatrixMode(MatrixMode::Model);
    sceGumTranslate(&(ScePspFVector3 { x: a.0, y: a.1, z: a.2 }));
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

// fn bind_texture(tex:&Texture) {
//   unsafe {
//     sceGuTexMode(TexturePixelFormat::Psm8888, 0, 0, true.into());
//     sceGuTexFunc(TextureEffect::Modulate, TextureColorComponent::Rgba);
//     sceGuTexFilter(TextureFilter::Nearest, TextureFilter::Nearest);
//     sceGuTexWrap(GuTexWrapMode::Repeat, GuTexWrapMode::Repeat);
//     sceGuTexImage(MipmapLevel::None, 
//       tex.pW, tex.pH, tex.pW, tex.data.cast_const().cast::<c_void>());
//   }
// }

// struct Texture {
//   width: i32, 
//   height: i32,
//   pW: i32, 
//   pH: i32,
//   data: *mut u8
// }

// fn load_texture(filename: &str) -> Texture {
//   load_texture_general(filename,false)
// }

// fn load_texture_flipped(filename: &str) -> Texture {
//   load_texture_general(filename,true)
// }

// fn load_texture_general(filename: &str, flipped:bool) -> Texture {
//   psp::dprintln!("loading texture...");
//   let f = File::open(filename,IoOpenFlags::RD_ONLY)
//     .expect("Texture not loaded");
  
//   let contents = f.bytes().collect::<Vec<u8>>();
//   unsafe {
//     stbi_set_flip_vertically_on_load(flipped.into());
//   }
//   let mut tex = create_texture(contents);

//   let size = (tex.pH * tex.pW * 4) as usize;

//   psp::dprintln!("creating data_buffer...");
//   let layout = Layout::from_size_align(
//     size,
//     16
//   ).expect("invalid layout for data_buffer"); 
//   let data_buffer = unsafe { alloc(layout) };

//   // // Copy to Data Buffer
//   copy_texture_data(
//     data_buffer.cast::<u32>(), 
//     tex.data.cast::<u32>(), 
//     tex.pW, 
//     tex.width, 
//     tex.height
//   );
  
//   // Free STB Data
//   unsafe { stbi_image_free(tex.data) };

//   let allocator = vram_alloc::get_vram_allocator().unwrap();

//   let swizzled_pixels = // if true {
//     allocator
//       .alloc_texture_pixels(tex.pW as u32, tex.pH as u32, TexturePixelFormat::Psm8888)
//       .as_mut_ptr_from_zero();
//   // } else {
//   //   unsafe { 
//   //     alloc(layout) 
//   //   }
//   // };
  
//   swizzle_fast(swizzled_pixels, data_buffer.cast::<u32>(), tex.pW * 4, tex.pH);

//   unsafe { dealloc(data_buffer,layout) };
//   tex.data = swizzled_pixels;

//   unsafe { sceKernelDcacheWritebackInvalidateAll() };

//   tex
// }

// fn create_texture(contents: Vec<u8>) -> Texture {
//     let mut width:i32 = 0;
//     let mut height:i32 = 0;
//     let mut nr_channels:i32 = 0;
//     let data = unsafe { 
//       stbi_load_from_memory(
//         contents.as_ptr(), 
//         contents.len() as i32, 
//         &mut width as *mut _, 
//         &mut height as *mut _, 
//         &mut nr_channels as *mut _, 
//         STBI_rgb_alpha
//       )
//     };
//     psp::dprintln!("loaded from memory: {:?}",data);


//     let mut tex = Texture {
//       width,
//       height,
//       pW: round_to_power_of_2(width),
//       pH: round_to_power_of_2(height),
//       data
//     };
//     tex
// }


// fn round_to_power_of_2(value: i32) -> i32 {
//   let mut poweroftwo = 1;
//   while poweroftwo < value {
//     poweroftwo <<= 1;
//   }
//   poweroftwo
// }

// fn copy_texture_data(dest: *mut u32, src: *const u32, power:i32, width:i32, height:i32){
//   for y in 0..height {
//     for x in 0..width {
//       unsafe {
//         let i = (x+y*power) as usize;
//         let j = (x+y*width) as usize;
//         *(dest.add(i)) = *(src.add(j));
//       }
//     }
//   }
// }

// fn swizzle_fast(out: *mut u8, input: *const u32, width: i32, height: i32) {
//   let width_blocks = width / 16;
//   let height_blocks = height / 8;

//   let src_pitch = (width - 16) / 4;
//   let src_row = width * 8;

//   let mut dst = out as *mut u32;
//   let mut ysrc = input as *const u8;

//   for _ in 0..height_blocks {
//       let mut xsrc = ysrc;
//       for _ in 0..width_blocks {
//           let mut src = xsrc as *const u32;
//           for _ in 0..8 {
//               unsafe {
//                   for _ in 0..4 {
//                       *dst = *src;
//                       src = src.add(1);
//                       dst = dst.add(1);
//                   }
//                   src = src.add(src_pitch as usize);
//               }
//           }
//           xsrc = unsafe { xsrc.add(16) };
//       }
//       ysrc = unsafe { ysrc.add(src_row as usize) };
//   }
// }