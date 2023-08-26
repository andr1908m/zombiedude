#![no_std]
#![no_main]

use core::ffi::c_void;

use psp::{sys::{self, TexturePixelFormat, sceGeEdramGetAddr, sceGuInit, GuContextType, GuState, DepthFunc, FrontFaceDirection, ShadingModel, GuSyncMode, GuSyncBehavior, sceGuDisplay, rgba, ClearBuffer}, Align16, vram_alloc::get_vram_allocator};

psp::module!("sample_cube", 1, 1);

#[repr(C, align(4))]
struct Vertex {
    u: f32,
    v: f32,
    x: f32,
    y: f32,
    z: f32,
}

const BUFFER_WIDTH: i32 = 512;
const SCREEN_WIDTH: i32 = 480;
const SCREEN_HEIGHT: i32 = 272;

static mut LIST: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);

fn list() -> *mut c_void {
  unsafe { &mut LIST as *mut _ as *mut c_void }
}

fn psp_main() {
  let _s = System::new();

  initialize_graphics();
  let mut running = true;
  while running {
    let frame = Frame::new();
    
    unsafe {
      let color = create_color(0x00, 0xFF, 0x00, 0xFF);
      clear_color(color);
    }

  }
  terminate_graphics();
}



fn create_color(r: u8, g: u8, b: u8, a: u8) -> u32 {
  rgba(r,g,b,a)
}



fn clear_color(color: u32) {
    unsafe { 
      sys::sceGuClearColor(color);
      sys::sceGuClear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::DEPTH_BUFFER_BIT);
    };
}

fn initialize_graphics() {
  let (draw_buff, disp_buff, depth_buff) = create_graphics_buffers();

  unsafe {  
    start_gu();
    start_display_context(draw_buff, disp_buff, depth_buff);
    set_viewport();
    set_depth_range();
    enable_scissors();
    enable_depth_test();
    enable_cull_face();
    enable_smooth_shading();
    enable_textures();
    send_to_gu();
    enable_display();
  }
}

fn terminate_graphics() {
  unsafe { sys::sceGuTerm() };
}

struct Frame {}

impl Frame {
  fn new() -> Self {
    unsafe { sys::sceGuStart(GuContextType::Direct, list()) };
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
    };
  }
}

fn enable_display() {
  unsafe { sys::sceGuDisplay(true) };
}

fn send_to_gu() {
  unsafe {
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait); // blocking call
    sys::sceDisplayWaitVblankStart();
  }
}

fn create_graphics_buffers() -> (*mut c_void, *mut c_void, *mut c_void) {
  let mut allocator = get_vram_allocator().unwrap();
  let bwi32 = BUFFER_WIDTH as u32;
  let shi32 = SCREEN_HEIGHT as u32;
  let draw_buff = allocator
    .alloc_texture_pixels(bwi32, shi32, TexturePixelFormat::Psm8888)
    .as_mut_ptr_from_zero() as *mut c_void;
  let disp_buff = allocator
    .alloc_texture_pixels(bwi32, shi32, TexturePixelFormat::Psm8888)
    .as_mut_ptr_from_zero() as *mut c_void;
  let depth_buff = allocator
    .alloc_texture_pixels(bwi32, shi32, TexturePixelFormat::Psm4444)
    .as_mut_ptr_from_zero() as *mut c_void;

  (draw_buff, disp_buff, depth_buff)
}

fn start_gu() {
  unsafe { sys::sceGuInit() };
}

fn start_display_context(draw_buff: *mut c_void, disp_buff: *mut c_void, depth_buff: *mut c_void) {
  unsafe {
    sys::sceGuStart(GuContextType::Direct, list());
    sys::sceGuDrawBuffer(sys::DisplayPixelFormat::Psm4444, draw_buff, BUFFER_WIDTH);
    sys::sceGuDispBuffer(SCREEN_WIDTH, SCREEN_HEIGHT, disp_buff, BUFFER_WIDTH);
    sys::sceGuDepthBuffer(depth_buff, BUFFER_WIDTH);
  }
}

fn set_viewport() {
  let x = 2048 - (SCREEN_WIDTH / 2) as u32;
  let y = 2048 - (SCREEN_HEIGHT / 2) as u32;
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
    sys::sceGuDepthFunc(DepthFunc::GreaterOrEqual);
  };
}

fn enable_cull_face() {
  unsafe { 
    sys::sceGuEnable(GuState::CullFace);
    sys::sceGuFrontFace(FrontFaceDirection::Clockwise);
  };
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

struct System {}

impl System {
  fn new() -> Self {
    psp::enable_home_button();
    Self {}
  }
}

impl Drop for System {
  fn drop(&mut self) {
    unsafe {
      sys::sceKernelExitGame();
    };
  }
}

fn initialize_system() {
  psp::enable_home_button();
}

fn exit_system() {
  unsafe {
    sys::sceKernelExitGame();
  }
}

