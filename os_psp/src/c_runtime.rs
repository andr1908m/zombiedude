mod c_runtime {
  extern crate alloc;
  use core;
  use psp::math::powf;
  use core::marker::*;

  pub trait One {
    fn one() -> Self;
  }

  impl One for i32 {
    fn one() -> Self {
      1
    }
  }

  impl One for u32 {
    fn one() -> Self {
      1
    }
  }

  pub unsafe fn post_inc<T: core::ops::AddAssign + One + Copy>(a: *mut T) -> T {
    let result: T = *a;
    *a += One::one();
    return result;
  }

  pub unsafe fn pre_inc<T: core::ops::AddAssign + One + Copy>(a: *mut T) -> T {
    *a += One::one();
    return *a;
  }

  pub unsafe fn pre_dec<T: core::ops::SubAssign + One + Copy>(a: *mut T) -> T {
    *a -= One::one();
    return *a;
  }

  pub unsafe fn pre_inc_ptr<T>(a: *mut *mut T) -> *mut T {
    *a = (*a).offset(1);
    return *a;
  }

  pub unsafe fn post_inc_ptr<T>(a: *mut *mut T) -> *mut T {
    let result: *mut T = *a;
    *a = (*a).offset(1);
    return result;
  }

  pub unsafe fn post_inc_const_ptr<T>(a: *mut *const T) -> *const T {
    let result: *const T = *a;
    *a = (*a).offset(1);
    return result;
  }

  pub unsafe fn memcpy(src: *mut u8, dest: *const u8, count: u64) {
    psp::dprintln!("gonna copy from dest {:?} to src {:?}", dest, src);
    dest.copy_to(src, count as usize);
    psp::dprintln!("copied from dest {:?} to src {:?}", dest, src);
  }

  pub unsafe fn memset(src: *mut u8, value: i32, count: u64) {
    core::ptr::write_bytes(src, value as u8, count as usize);
  }

  pub unsafe fn malloc(count: u64) -> *mut u8 {
    let layout = core::alloc::Layout::from_size_align(count as usize, 1)
      .expect("Bad layout");

    return alloc::alloc::alloc(layout);
  }

  pub unsafe fn realloc<T>(data: *mut T, count: u64) -> *mut u8 {
    if data.is_null() {
      return malloc(count);
    }

    let layout = core::alloc::Layout::from_size_align(count as usize, 1)
      .expect("Bad layout");

    return alloc::alloc::realloc(data as *mut u8, layout, count as usize);
  }

  pub unsafe fn free<T>(data: *mut T) {
    let layout = core::alloc::Layout::from_size_align(1, 1)
      .expect("Bad layout");

    alloc::alloc::dealloc(data as *mut u8, layout);
  }

  pub fn _lrotl(x: u32, y: i32) -> u32 {
    return (x << y) | (x >> (32 - y));
  }

  pub fn abs(x: i32) -> i32 {
    return i32::abs(x);
  }

  pub fn pow(x: f32, p: f32) -> f32 {
    powf(x, p)
  }
}
