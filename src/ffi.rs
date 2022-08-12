#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::{c_void, CStr, CString};

pub type c_char = i8;

pub fn input(prompt: &str) -> Option<String> {
    let cprompt = CString::new(prompt.as_bytes()).expect("utf8");
    let s = unsafe { linenoise(cprompt.as_ptr()) };

    if s != 0 as *mut c_char {
        unsafe { linenoiseHistoryAdd(s) };
        let rs = unsafe { CStr::from_ptr(s) }
            .to_str()
            .expect("utf8")
            .to_string();
        unsafe { linenoiseFree(s as *mut c_void) };
        Some(rs)
    } else {
        None
    }
}

extern "C" {
    pub fn linenoise(prompt: *const c_char) -> *mut c_char;
    pub fn linenoiseHistoryAdd(line: *const c_char) -> i32;
    pub fn linenoiseFree(p: *mut c_void);
}
