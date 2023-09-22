#![no_std]

use os_psp::psp_extern;

psp_extern! {
  #![name = "test_create"]
  #![flags = 0x0]
  #![version = (0x00, 0x01)]

  #[psp(0x0845f1cf)]
  pub fn library_call();

  #[psp(0x28fa2125)]
  pub fn library_call_2(b:i32);
}