use psp::sys;

use psp;

pub(crate) struct System {}

impl System {
  pub(crate) fn new() -> Self {
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
