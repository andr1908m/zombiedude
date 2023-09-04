use psp::sys::*;

use crate::c_compat::ToVoid;

pub struct File {
  handle: SceUid
}

impl File {
  pub fn open(path: &str, flags: IoOpenFlags) -> Result<Self, i32> {
    unsafe {
      let handle = sceIoOpen(Cstr::new(path).as_ptr(), flags, 0777);
      if handle.0 < 0 {
        Err(handle.0)
      } else {
        Ok(Self { handle })
      }
    }
  }

  pub fn bytes(&self) -> ByteIterator {
    ByteIterator { file:self, bytes_read:0 }
  }

  pub fn chars(&self) -> CharIterator {
    CharIterator { file:self }
  }

  pub fn read_byte(&self) -> Option<u8> {
    unsafe {
      let mut v:u8 = 0;
      let bytes_read = sceIoRead(self.handle, v.as_mut_void_ptr(), 1);
    
      if bytes_read == 0 {
        None
      } else {
        Some(v)
      }
    }
  }

  pub fn read_char(&self) -> Option<char> {
    unsafe {
      let mut v:char = ' ';
      let bytes_read = sceIoRead(self.handle, v.as_mut_void_ptr(), 1);
      if bytes_read == 0 {
        None
      } else {
        Some(v)
      }
    }
  }

  pub fn write(&self, data: &str) {
    unsafe {
      let v = Cstr::new(data);
      sceIoWrite(self.handle, v.as_ptr(), v.len());
    }
  }

  fn close(&self) {
    unsafe {
      sceIoClose(self.handle);
    }
  }
}

impl Drop for File {
  fn drop(&mut self) {
    self.close()
  }
}

pub struct ByteIterator<'a> {
  file: &'a File,
  bytes_read: i32
}

impl<'a> Iterator for ByteIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
      let v = self.file.read_byte();
      self.bytes_read += 1;
      // if self.bytes_read % (1024 * 10) == 0 {
      //   psp::dprintln!("read {} bytes so far", self.bytes_read);
      // }
      v
    }
}

pub struct CharIterator<'a> {
  file: &'a File
}

impl<'a> Iterator for CharIterator<'a> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    self.file.read_char()
  }
}

pub struct Cstr<'a> {
  data: &'a str,
  length: usize,
}

impl<'a> Cstr<'a> {
  pub fn new(s: &'a str) -> Self {
    Self {
      data: s,
      length: s.len(),
    }
  }

  pub fn as_ptr<T>(&self) -> *const T {
    [self.data, "\0"].concat().as_ptr() as *const T
  }

  pub fn len(&self) -> usize {
    self.length
  }
}
