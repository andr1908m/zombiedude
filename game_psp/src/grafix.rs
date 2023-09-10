use psp::Align16;
use psp::sys::*;
use psp::vram_alloc::*;

use core::ffi::c_void;
use core::marker::PhantomData;
use crate::c_compat::ToVoid;

use psp::sys;

const BUFFER_WIDTH: i32 = 512;
const SCREEN_WIDTH: i32 = 480;
const SCREEN_HEIGHT: i32 = 272;

const _256_KIBIBYTES: usize = 0x40000;

impl ToVoid for VramMemChunk<'_> {
  fn as_void_ptr(&self) -> *const c_void {
    self.as_mut_ptr_from_zero() as *const c_void
  }

  fn as_mut_void_ptr(&mut self) -> *mut c_void {
    self.as_mut_ptr_from_zero() as *mut c_void
  }
}

pub struct Graphics {
  list: Align16<[u32; _256_KIBIBYTES]>,
  draw_buffer:Option<*mut c_void>,
  disp_buffer:Option<*mut c_void>,
  depth_buffer:Option<*mut c_void>,
  allocator:SimpleVramAllocator,
}

impl Graphics {
  pub fn for_each_frame(&mut self, cb: fn()) {
    loop {
      self.start_display_list();
      unsafe {
        sys::sceGuDisable(GuState::DepthTest);
      }
      cb();
      unsafe {
        sys::sceGuFinish();
        sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
        sys::sceDisplayWaitVblankStart();
        sys::sceGuSwapBuffers();
      }
    }
  }

  pub fn new() -> Self {
    let mut this = Self::create_defaults();
    this.initialize_texture_buffers();
    this.setup_graphics_engine();
    this.specify_bounds();
    this.enable_features();
    this.send_to_graphics_engine();
    
    this
  }

  fn create_defaults() -> Graphics {
    Self {
      list: Align16([0; _256_KIBIBYTES]),
      draw_buffer: None,
      disp_buffer: None,
      depth_buffer: None,
      allocator: get_vram_allocator().unwrap(),
    }
  }

  fn initialize_texture_buffers(&mut self) {
    self.draw_buffer = self.create_texture_buffer(TexturePixelFormat::Psm8888);
    self.disp_buffer = self.create_texture_buffer(TexturePixelFormat::Psm8888);
    self.depth_buffer = self.create_texture_buffer(TexturePixelFormat::Psm4444);
  }

  fn create_texture_buffer(&self, format:TexturePixelFormat) -> Option<*mut c_void> {
    let width = BUFFER_WIDTH as u32;
    let height = SCREEN_HEIGHT as u32;
    let ptr = self.allocator
      .alloc_texture_pixels(width, height, format)
      .as_mut_void_ptr();
    Some(ptr)
  }
  
  fn setup_graphics_engine(&mut self) {
    start_graphics_system();
    self.start_display_context();
  }

  fn start_display_context(&mut self) {
    self.start_display_list();
    self.specify_graphics_buffers();
  }

  fn start_display_list(&mut self) {
    unsafe { 
      sys::sceGuStart(GuContextType::Direct, self.list.as_mut_void_ptr()) 
    };
  }

  fn specify_graphics_buffers(&self) {
    self.specify_draw_buffer();
    self.specify_disp_buffer();
    self.specify_depth_buffer();
  }

  fn specify_draw_buffer(&self) {
    unsafe {
      sys::sceGuDrawBuffer(
        DisplayPixelFormat::Psm4444, 
        self.draw_buffer.unwrap(), 
        BUFFER_WIDTH
      );
    }
  }

  fn specify_disp_buffer(&self) {
    unsafe {
      sys::sceGuDispBuffer(
        SCREEN_WIDTH, 
        SCREEN_HEIGHT, 
        self.disp_buffer.unwrap(), 
        BUFFER_WIDTH
      );
    }
  }

  fn specify_depth_buffer(&self) {
    unsafe {
      sys::sceGuDepthBuffer(
        self.depth_buffer.unwrap(), 
        BUFFER_WIDTH
      );
    }
  }
  
  fn specify_bounds(&self) {
    specify_viewport();
    specify_depth_range();
  }

  fn enable_features(&self) {
    enable_viewport_scissors();
    enable_depth_test();
    enable_face_culling();
    enable_smooth_shading();
    enable_textures();
  }

  fn send_to_graphics_engine(&self) {
    execute_display_list();
    enable_display();
  }
}

impl Drop for Graphics {
  fn drop(&mut self) {
    unsafe { 
      sys::sceGuTerm();
      self.allocator.free_all(); 
    };
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

fn start_graphics_system() {
  unsafe { 
    sys::sceGuInit();
  }
}

fn specify_viewport() {
  let x = 2048 - ((SCREEN_WIDTH / 2) as u32);
  let y = 2048 - ((SCREEN_HEIGHT / 2) as u32);
  unsafe {
    sys::sceGuOffset(x, y);
    sys::sceGuViewport(2048, 2048, SCREEN_WIDTH, SCREEN_HEIGHT);
  }
}

fn specify_depth_range() {
  unsafe {
    sys::sceGuDepthRange(65535, 0);
  }
}

/// enable cutting away things outside the viewport
fn enable_viewport_scissors() {
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
  }
}

fn enable_face_culling() {
  unsafe {
    sys::sceGuEnable(GuState::CullFace);
    sys::sceGuFrontFace(FrontFaceDirection::Clockwise);
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
