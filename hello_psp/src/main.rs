#![no_std]
#![no_main]
//#![feature(generators, generator_trait)]
//#![feature(restricted_std)]
//use core::ops::{Generator, GeneratorState};

extern crate alloc;
//extern crate libc;

mod io;

use alloc::{
    borrow::ToOwned,
    fmt::format,
    string::{String, ToString},
    vec,
};
use psp::sys::{
    self, DepthFunc, DisplayPixelFormat, FrontFaceDirection, GuContextType, GuState,
    GuSyncBehavior, GuSyncMode, ShadingModel, SystemParamLanguage, UtilityDialogButtonAccept,
    UtilityDialogCommon, UtilityMsgDialogMode, UtilityMsgDialogOption, UtilityMsgDialogParams,
    UtilityMsgDialogPressed,
};

use core::{
    ffi::c_void,
    fmt::{Error, Write},
};

psp::module!("sample_module", 1, 1);

static mut LIST: psp::Align16<[u32; 262144]> = psp::Align16([0; 262144]);
const SCR_WIDTH: i32 = 480;
const SCR_HEIGHT: i32 = 272;
const BUF_WIDTH: i32 = 512;

fn setup_gu() {
    unsafe {
        sys::sceGuInit();
        sys::sceGuStart(GuContextType::Direct, &mut LIST as *mut _ as *mut c_void);
        sys::sceGuDrawBuffer(
            DisplayPixelFormat::Psm8888,
            core::ptr::null_mut(),
            BUF_WIDTH,
        );
        sys::sceGuDispBuffer(SCR_WIDTH, SCR_HEIGHT, 0x88000 as *mut c_void, BUF_WIDTH);
        sys::sceGuDepthBuffer(0x110000 as *mut c_void, BUF_WIDTH);
        sys::sceGuOffset(
            2048 - (SCR_WIDTH as u32 / 2),
            2048 - (SCR_HEIGHT as u32 / 2),
        );
        sys::sceGuViewport(2048, 2048, SCR_WIDTH, SCR_HEIGHT);
        sys::sceGuDepthRange(0xc350, 0x2710);
        sys::sceGuScissor(0, 0, SCR_WIDTH, SCR_HEIGHT);
        sys::sceGuEnable(GuState::ScissorTest);
        sys::sceGuDepthFunc(DepthFunc::GreaterOrEqual);
        sys::sceGuEnable(GuState::DepthTest);
        sys::sceGuFrontFace(FrontFaceDirection::Clockwise);
        sys::sceGuShadeModel(ShadingModel::Smooth);
        sys::sceGuEnable(GuState::CullFace);
        sys::sceGuEnable(GuState::ClipPlanes);
        sys::sceGuFinish();
        sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);

        sys::sceDisplayWaitVblankStart();
        sys::sceGuDisplay(true);
    }
}

fn total_memory() -> usize {
    unsafe { psp::sys::sceKernelTotalFreeMemSize() }
}

fn psp_main() {
    psp::enable_home_button();

    setup_gu();
    let kb = total_memory() / 1024;
    let mb_rem = kb % 1024;
    let mb = kb / 1024;
    let mb_str: &str = &mb.to_string();
    let mb_rem_str: &str = &mb_rem.to_string();
    print_dialog(&["mb: ", &mb_str, ".", &mb_rem_str].concat());
    use alloc::format;
    let s = format!("{}, {}", "hi", 2);
    print_dialog(&s);
    let my_string = my_string();
    print_dialog(&my_string);
    let v = vec![1, 2, 3];
    for d in v {
        print_dialog(&d.to_string());
    }
}

fn my_string() -> String {
    let mut s = "hello".to_owned();
    s.write_str(" world");
    s.write_str("!");
    s
}

#[cfg(test)]
mod test {
    extern crate std;
    use super::*;
    use std::prelude::*;

    #[test]
    fn my_string_test() {
        let v = my_string();
        assert_eq!(v, "hello world!");
    }
}

fn print_dialog(text: &str) {
    assert!(text.is_ascii());
    assert!(text.len() <= 512);

    let dialog_size = core::mem::size_of::<UtilityMsgDialogParams>();
    let base = UtilityDialogCommon {
        size: dialog_size as u32,
        language: SystemParamLanguage::English,
        button_accept: UtilityDialogButtonAccept::Cross, // X to accept
        graphics_thread: 0x11,                           // magic number stolen from pspsdk example
        access_thread: 0x13,
        font_thread: 0x12,
        sound_thread: 0x10,
        result: 0,
        reserved: [0i32; 4],
    };
    let mut message = [0u8; 512];
    message[..text.len()].copy_from_slice(text.as_bytes());
    let mut msg_dialog = UtilityMsgDialogParams {
        base,
        unknown: 0,
        mode: UtilityMsgDialogMode::Text,
        error_value: 0,
        message,
        options: UtilityMsgDialogOption::TEXT,
        button_pressed: UtilityMsgDialogPressed::Unknown1,
    };
    unsafe {
        sys::sceUtilityMsgDialogInitStart(&mut msg_dialog as *mut UtilityMsgDialogParams);
    }
    loop {
        let status = unsafe { sys::sceUtilityMsgDialogGetStatus() };
        match status {
            2 => unsafe { sys::sceUtilityMsgDialogUpdate(1) },
            3 => unsafe { sys::sceUtilityMsgDialogShutdownStart() },
            0 => break,
            _ => (),
        }
        unsafe {
            sys::sceGuStart(GuContextType::Direct, &mut LIST as *mut _ as *mut c_void);
            sys::sceGuFinish();
            sys::sceGuSync(GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
            sys::sceDisplayWaitVblankStart();
            sys::sceGuSwapBuffers();
        }
    }
}
