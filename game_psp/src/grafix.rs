use psp::Align16;
use psp::sys::*;
use psp::vram_alloc;

use core::ffi::c_void;
use crate::c_compat::ToVoid;

use psp::sys;

const BUFFER_WIDTH: i32 = 512;
const SCREEN_WIDTH: i32 = 480;
const SCREEN_HEIGHT: i32 = 272;

static mut DISPLAY_LIST: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);

pub struct Graphics {}

impl Graphics {
  pub fn new() -> Self {
    Self::setup_graphics_unit();
    Self::set_bounds();
    Self::enable_features();
    Self::send_to_graphics_unit();
    
    Self {}
  }
  
  fn setup_graphics_unit() {
    let (draw_buff, disp_buff, depth_buff) = create_graphics_buffers();
    start_gu();
    start_display_context(draw_buff, disp_buff, depth_buff);
  }
  
  fn set_bounds() {
    set_viewport();
    set_depth_range();
  }

  fn enable_features() {
    enable_scissors();
    enable_depth_test();
    enable_cull_face();
    enable_smooth_shading();
    enable_textures();
  }

  fn send_to_graphics_unit() {
    execute_display_list();
    enable_display();
  }
}

impl Drop for Graphics {
  fn drop(&mut self) {
    unsafe { sys::sceGuTerm() };
  }
}
pub struct Frame {}

impl Frame {
  pub fn new() -> Self {
    unsafe {
      sys::sceGuStart(GuContextType::Direct, DISPLAY_LIST.as_mut_void_ptr());
    }
    Self {}
  }
}

impl Drop for Frame {
  fn drop(&mut self) {
    unsafe {
      sys::sceGuFinish();
      sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
      sys::sceDisplayWaitVblankStart();
      sys::sceGuSwapBuffers();
    }
  }
}

fn enable_display() {
  unsafe {
    let _s = sys::sceGuDisplay(true);
  }
}

fn execute_display_list() {
  unsafe {
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait); // blocking call
    sys::sceDisplayWaitVblankStart();
  }
}

fn create_graphics_buffers() -> (*mut c_void, *mut c_void, *mut c_void) {
  let mut allocator = vram_alloc::get_vram_allocator().unwrap();
  let bwu32 = BUFFER_WIDTH as u32;
  let shu32 = SCREEN_HEIGHT as u32;
  let draw_buff = allocator
    .alloc_texture_pixels(bwu32, shu32, TexturePixelFormat::Psm8888)
    .as_mut_ptr_from_zero() as *mut c_void;
  let disp_buff = allocator
    .alloc_texture_pixels(bwu32, shu32, TexturePixelFormat::Psm8888)
    .as_mut_ptr_from_zero() as *mut c_void;
  let depth_buff = allocator
    .alloc_texture_pixels(bwu32, shu32, TexturePixelFormat::Psm4444)
    .as_mut_ptr_from_zero() as *mut c_void;

  (draw_buff, disp_buff, depth_buff)
}

fn start_gu() {
  unsafe { sys::sceGuInit() }
}

fn start_display_context(draw_buff: *mut c_void, disp_buff: *mut c_void, depth_buff: *mut c_void) {
  unsafe {
    sys::sceGuStart(GuContextType::Direct, DISPLAY_LIST.as_mut_void_ptr());
    sys::sceGuDrawBuffer(DisplayPixelFormat::Psm4444, draw_buff, BUFFER_WIDTH);
    sys::sceGuDispBuffer(SCREEN_WIDTH, SCREEN_HEIGHT, disp_buff, BUFFER_WIDTH);
    sys::sceGuDepthBuffer(depth_buff, BUFFER_WIDTH);
  }
}

fn set_viewport() {
  let x = 2048 - ((SCREEN_WIDTH / 2) as u32);
  let y = 2048 - ((SCREEN_HEIGHT / 2) as u32);
  unsafe {
    sys::sceGuOffset(x, y);
    sys::sceGuViewport(2048, 2048, SCREEN_WIDTH, SCREEN_HEIGHT);
  }
}

fn set_depth_range() {
  unsafe {
    sys::sceGuDepthRange(65535, 0);
  }
}

fn enable_scissors() {
  unsafe {
    sys::sceGuEnable(GuState::ScissorTest);
    sys::sceGuScissor(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
  }
}

/// Things further away should be clipped
fn enable_depth_test() {
  unsafe {
    sys::sceGuEnable(GuState::DepthTest);
    sys::sceGuDepthFunc(sys::DepthFunc::GreaterOrEqual);
  }
}

fn enable_cull_face() {
  unsafe {
    sys::sceGuEnable(GuState::CullFace);
    sys::sceGuFrontFace(sys::FrontFaceDirection::Clockwise);
  }
}

fn enable_textures() {
  unsafe {
    sys::sceGuEnable(GuState::Texture2D);
    sys::sceGuEnable(GuState::ClipPlanes);
  }
}

fn enable_smooth_shading() {
  unsafe {
    sys::sceGuShadeModel(ShadingModel::Smooth);
  }
}
