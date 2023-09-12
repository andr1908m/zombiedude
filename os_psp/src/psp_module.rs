
use core::ffi::c_void;

#[macro_export]
macro_rules! psp_export {
  ($name:expr,$version:expr, $(($nid:expr, $lib:ident)),*) => {
    const NAME:&'static str = $name;
    const LIB_COUNT:u16 = [$($nid),*].len() as u16;

    extern "C" {
      static _gp: u8;
      static __lib_ent_bottom: u8;
      static __lib_ent_top: u8;
      static __lib_stub_bottom: u8;
      static __lib_stub_top: u8;
    }

    #[no_mangle]
    #[link_section = ".rodata.sceModuleInfo"]
    #[used]
    static MODULE_INFO: psp::Align16<psp::sys::SceModuleInfo> =
      psp::Align16(psp::sys::SceModuleInfo {
        mod_attribute: 0,
        mod_version: [$version.0, $version.1],
        mod_name: psp::sys::SceModuleInfo::name(NAME),
        terminal: 0,
        gp_value: unsafe { &_gp },
        stub_top: unsafe { &__lib_stub_top },
        stub_end: unsafe { &__lib_stub_bottom },
        ent_top: unsafe { &__lib_ent_top },
        ent_end: unsafe { &__lib_ent_bottom },
      });

    use core::ffi::c_void;

    #[no_mangle]
    extern "C" fn module_start(_argc: usize, _argv: *mut c_void) -> isize {
      0
    }

    #[no_mangle]
    #[link_section = ".rodata.sceResident"]
    #[used]
    static SYSLIB_EXPORTS: psp::sys::SceLibraryEntryTable = 
      psp::sys::SceLibraryEntryTable {
        module_start_nid: 0xd632acdb, // module_start
        module_info_nid: 0xf01d73a7,  // SceModuleInfo
        module_start: module_start,
        module_info: &MODULE_INFO.0,
      };
      use core::marker::Sync;

      struct Export(*const ());
      unsafe impl Sync for Export {}

    #[no_mangle]
    #[link_section = ".rodata.sceResident"]
    static MYLIB_EXPORTS: [Export; LIB_COUNT as usize * 2] = [
      $(Export($nid as *const ()),)*
      $(Export($lib as *const ()),)*
    ];

    #[repr(C, packed)]
    struct LibraryEntry {
      name: *const u8,
      version: (u8, u8),
      attribute: psp::sys::SceLibAttr,
      entry_len: u8,
      var_count: u8,
      func_count: u16,
      entry_table: *const c_void,
    }

    unsafe impl Sync for LibraryEntry {}

    #[no_mangle]
    #[link_section = ".lib.ent"]
    #[used]
    static LIB_ENT: [LibraryEntry; 2] = [
      LibraryEntry {
        // TODO: Fix this?
        name: core::ptr::null(),
        version: $version,
        attribute: psp::sys::SceLibAttr::SCE_LIB_IS_SYSLIB,
        entry_len: 4,
        var_count: 1,
        func_count: 1,
        entry_table: &SYSLIB_EXPORTS as *const _ as *const c_void,
      },
      LibraryEntry {
        // TODO: Fix this?
        name: core::ptr::null(),
        version: $version,
        attribute: psp::sys::SceLibAttr::SCE_LIB_AUTO_EXPORT,
        entry_len: 4,
        var_count: 0,
        func_count: LIB_COUNT,
        entry_table: &MYLIB_EXPORTS as *const _ as *const c_void,
      }
    ];
  };
}
pub struct Args {
  count: usize,
  values: *mut u8
}

impl Args {
  fn new(argc: usize, argv: *mut c_void) -> Args {
    Self {
      count:argc,
      values:argv as *mut u8
    }
  }
  pub fn at(&self, i: usize) -> Option<u8> {
    if i > self.count {
      None
    } else {
      unsafe {
        Some(*self.values.add(i))
      }
    } 
  }
}