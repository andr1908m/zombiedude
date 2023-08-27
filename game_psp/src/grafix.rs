use psp::Align16;
use psp::vram_alloc;

use core::ffi::c_void;

use psp::sys;

const BUFFER_WIDTH: i32 = 512;
const SCREEN_WIDTH: i32 = 480;
const SCREEN_HEIGHT: i32 = 272;

static mut LIST: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);

fn list() -> *mut c_void {
  unsafe { &mut LIST as *mut _ as *mut c_void }
}

pub struct Graphics {}

impl Graphics {
  pub fn new() -> Self {
    Self::setup_gu();
    Self::set_bounds();
    Self::enable_features();
    Self::send_to_gu();
    
    Self {}
  }
  
  fn setup_gu() {
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

  fn send_to_gu() {
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
      sys::sceGuStart(sys::GuContextType::Direct, list());
    }
    Self {}
  }
}

impl Drop for Frame {
  fn drop(&mut self) {
    unsafe {
      sys::sceGuFinish();
      sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
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
    sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait); // blocking call
    sys::sceDisplayWaitVblankStart();
  }
}

fn create_graphics_buffers() -> (*mut c_void, *mut c_void, *mut c_void) {
  let mut allocator = vram_alloc::get_vram_allocator().unwrap();
  let bwu32 = BUFFER_WIDTH as u32;
  let shu32 = SCREEN_HEIGHT as u32;
  let draw_buff = allocator
    .alloc_texture_pixels(bwu32, shu32, sys::TexturePixelFormat::Psm8888)
    .as_mut_ptr_from_zero() as *mut c_void;
  let disp_buff = allocator
    .alloc_texture_pixels(bwu32, shu32, sys::TexturePixelFormat::Psm8888)
    .as_mut_ptr_from_zero() as *mut c_void;
  let depth_buff = allocator
    .alloc_texture_pixels(bwu32, shu32, sys::TexturePixelFormat::Psm4444)
    .as_mut_ptr_from_zero() as *mut c_void;

  (draw_buff, disp_buff, depth_buff)
}

fn start_gu() {
  unsafe { sys::sceGuInit() }
}

fn start_display_context(draw_buff: *mut c_void, disp_buff: *mut c_void, depth_buff: *mut c_void) {
  unsafe {
    sys::sceGuStart(sys::GuContextType::Direct, list());
    sys::sceGuDrawBuffer(sys::DisplayPixelFormat::Psm4444, draw_buff, BUFFER_WIDTH);
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
    sys::sceGuEnable(sys::GuState::ScissorTest);
    sys::sceGuScissor(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
  }
}

/// Things further away should be clipped
fn enable_depth_test() {
  unsafe {
    sys::sceGuEnable(sys::GuState::DepthTest);
    sys::sceGuDepthFunc(sys::DepthFunc::GreaterOrEqual);
  }
}

fn enable_cull_face() {
  unsafe {
    sys::sceGuEnable(sys::GuState::CullFace);
    sys::sceGuFrontFace(sys::FrontFaceDirection::Clockwise);
  }
}

fn enable_textures() {
  unsafe {
    sys::sceGuEnable(sys::GuState::Texture2D);
    sys::sceGuEnable(sys::GuState::ClipPlanes);
  }
}

fn enable_smooth_shading() {
  unsafe {
    sys::sceGuShadeModel(sys::ShadingModel::Smooth);
  }
}
