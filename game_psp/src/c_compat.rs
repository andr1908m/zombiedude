use psp::Align16;

use core::ffi::c_void;

pub(crate) trait ToVoid {
  fn as_void_ptr(&self) -> *const c_void;
  fn as_mut_void_ptr(&mut self) -> *mut c_void;
}

impl<T> ToVoid for Align16<T> {
  fn as_void_ptr(&self) -> *const c_void {
    self as *const _ as *const c_void
  }

  fn as_mut_void_ptr(&mut self) -> *mut c_void {
    self as *mut _ as *mut c_void
  }
}
