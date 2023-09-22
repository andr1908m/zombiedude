
use core::ffi::c_void;

pub mod module_info_attr {
  pub const USER: u16 = 0;
  pub const NO_STOP: u16 = 0x0001;
  pub const SINGLE_LOAD: u16 = 0x0002;
  pub const SINGLE_START: u16 = 0x0004;
  pub const KERNEL: u16 = 0x1000;
}

#[macro_export]
macro_rules! psp_module {
    ($name:expr, $attributes:expr, $version_major:expr, $version_minor: expr) => {
        #[doc(hidden)]
        pub mod __psp_module {
            use super::*;
            #[no_mangle]
            #[link_section = ".rodata.sceModuleInfo"]
            #[used]
            static MODULE_INFO: psp::Align16<psp::sys::SceModuleInfo> =
                psp::Align16(psp::sys::SceModuleInfo {
                    mod_attribute: $attributes,
                    mod_version: [$version_major, $version_minor],
                    mod_name: psp::sys::SceModuleInfo::name($name),
                    terminal: 0,
                    gp_value: unsafe { &_gp },
                    stub_top: unsafe { &__lib_stub_top },
                    stub_end: unsafe { &__lib_stub_bottom },
                    ent_top: unsafe { &__lib_ent_top },
                    ent_end: unsafe { &__lib_ent_bottom },
                });

            extern "C" {
                static _gp: u8;
                static __lib_ent_bottom: u8;
                static __lib_ent_top: u8;
                static __lib_stub_bottom: u8;
                static __lib_stub_top: u8;
            }

            #[no_mangle]
            #[link_section = ".lib.ent"]
            #[used]
            static LIB_ENT: psp::sys::SceLibraryEntry = psp::sys::SceLibraryEntry {
                // TODO: Fix this?
                name: core::ptr::null(),
                version: ($version_major, $version_minor),
                attribute: psp::sys::SceLibAttr::SCE_LIB_IS_SYSLIB,
                entry_len: 4,
                var_count: 1,
                func_count: 1,
                entry_table: &LIB_ENT_TABLE,
            };

            #[no_mangle]
            #[link_section = ".rodata.sceResident"]
            #[used]
            static LIB_ENT_TABLE: psp::sys::SceLibraryEntryTable =
                psp::sys::SceLibraryEntryTable {
                    module_start_nid: 0xd632acdb, // module_start
                    module_info_nid: 0xf01d73a7,  // SceModuleInfo
                    module_start: module_start,
                    module_info: &MODULE_INFO.0,
                };

            use core::ffi::c_void;
            use os_psp::psp_module::module_info_attr;

            #[no_mangle]
            extern "C" fn module_start(argc_bytes: usize, argv: *mut c_void) -> isize {
              psp::dprintln!("hello from main thread!");
                // extern "C" fn main_thread(argc: usize, argv: *mut c_void) -> i32 {
                  
                //     let kernel_mode = MODULE_INFO.0.mod_attribute & (module_info_attr::KERNEL);
                //     let psp_main = 
                //       if kernel_mode == 0 {
                //         super::psp_main as fn()
                //       } else { 
                //         let as_u32 = super::psp_main 
                //           as fn() 
                //           as *const c_void
                //           as u32;
                //         let as_void_ptr = (as_u32| 0x80000000) as *const c_void;
                //         unsafe {
                //           core::mem::transmute::<*const c_void, fn()>(as_void_ptr)
                //         } 
                //     };
                    
                //     psp::_start!(psp_main, argc, argv)
                // }

                // main_thread(argc_bytes,argv);

                // unsafe {
                //     let id = psp::sys::sceKernelCreateThread(
                //         b"main_thread\0".as_ptr(),
                //         main_thread,
                //         // default priority of 32.
                //         32,
                //         // 256kb stack
                //         256 * 1024,
                //         psp::sys::ThreadAttributes::USER | psp::sys::ThreadAttributes::VFPU,
                //         core::ptr::null_mut(),
                //     );

                //     psp::sys::sceKernelStartThread(id, argc_bytes, argv);
                // }

                0
            }
        }
    };
}

const fn name_as_bytes<const T: usize>(name: &str) -> [u8; T] {
  let mut buf = [0; T];

  let name_bytes = name.as_bytes();
  let mut i = 0;

  while i < name_bytes.len() {
      buf[i] = name_bytes[i];
      i += 1;
  }

  buf
}

pub const fn as_bytes<const T: usize>(name: &str) -> [u8; T] {
  let mut buf = [0; T];

  let name_bytes = name.as_bytes();
  let mut i = 0;

  while i < name_bytes.len() {
      buf[i] = name_bytes[i];
      i += 1;
  }

  buf
}

#[macro_export]
macro_rules! psp_export {
  (
    #![module_name = $name:expr]
    #![version = $version:expr]

    #![lib_name = $lib_name:expr]
    $($nid:expr => $lib:ident);*;
  ) => {
    #[doc(hidden)]
    mod __psp_module {
      use super::*;
      use psp::Align16;
      use psp::sys::SceModuleInfo;
      use core::ffi::c_void;
      use psp::sys::sceKernelSleepThread;
      use psp::sys::SceLibraryEntryTable;
      use core::marker::Sync;
      use psp::sys::SceLibAttr;

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
      static MODULE_INFO: Align16<SceModuleInfo> =
        Align16(SceModuleInfo {
          mod_attribute: 0,
          mod_version: [$version.0, $version.1],
          mod_name: SceModuleInfo::name(NAME),
          terminal: 0,
          gp_value: unsafe { &_gp },
          stub_top: unsafe { &__lib_stub_top },
          stub_end: unsafe { &__lib_stub_bottom },
          ent_top: unsafe { &__lib_ent_top },
          ent_end: unsafe { &__lib_ent_bottom },
        });

      #[no_mangle]
      extern "C" fn module_start(_argc: usize, _argv: *mut c_void) -> isize {
        psp::dprintln!("hello from psp_module!");
        0
      }
      
      #[no_mangle]
      #[link_section = ".rodata.sceResident"]
      #[used]
      static SYSLIB_EXPORTS: SceLibraryEntryTable = 
        SceLibraryEntryTable {
          module_start_nid: 0xd632acdb, // module_start
          module_info_nid: 0xf01d73a7,  // SceModuleInfo
          module_start: module_start,
          module_info: &MODULE_INFO.0,
        };
      
      struct Export(*const c_void);
      unsafe impl Sync for Export {}

      const EXPORT_COUNT:usize = LIB_COUNT as usize * 2;

      #[no_mangle]
      #[link_section = ".rodata.sceResident"]
      #[used]
      static MYLIB_EXPORTS: [Export; EXPORT_COUNT] = [
        $(Export($nid as *const c_void),)*
        $(Export($lib as *const c_void),)*
      ];

      #[repr(C)]
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

      const LIB_NAME:*const u8 = $crate::psp_module::as_bytes(
        concat!($lib_name, "\0")).as_ptr();

      #[no_mangle]
      #[link_section = ".lib.ent"]
      #[used]
      static LIB_ENT: [LibraryEntry; 2] = [
        LibraryEntry {
          name: core::ptr::null(),
          version: $version,
          attribute: SceLibAttr::SCE_LIB_IS_SYSLIB,
          entry_len: 4,
          var_count: 1,
          func_count: 1,
          entry_table: &SYSLIB_EXPORTS as *const _ as *const c_void,
        },
        LibraryEntry {
          name: LIB_NAME,
          version: $version,
          attribute: SceLibAttr::SCE_LIB_AUTO_EXPORT,
          entry_len: EXPORT_COUNT as u8,
          var_count: 0,
          func_count: LIB_COUNT,
          entry_table: &MYLIB_EXPORTS as *const _ as *const c_void,
        }
      ];
    }
  }
}
