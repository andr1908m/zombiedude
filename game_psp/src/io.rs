use psp::sys::{sceIoClose, sceIoOpen, sceIoWrite, IoOpenFlags, SceUid};

pub struct File {
    handle: SceUid,
}

impl File {
    pub fn open(path: &str, flags: IoOpenFlags) -> Self {
        unsafe {
            let handle = sceIoOpen(Cstr::new(path).as_ptr(), flags, 0777);
            assert!(handle.0 >= 0, "error code: {}", handle.0);
            Self { handle }
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
