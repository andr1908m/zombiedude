#![no_main]
#![no_std]

use os_psp::psp_export;

fn library_call() {
  psp::dprintln!("Hello, world from library_call");
}

fn library_call_2(_b:i32) {
  psp::dprintln!("Hello, world from library_call_2, called with {}", _b);
}

psp_export! {
  "test_create",
  (0,1), 
  (0x0845f1cf, library_call), 
  (0x28fa2125, library_call_2)
}
