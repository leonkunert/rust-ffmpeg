#![crate_name = "avdevice54"]
#![crate_type = "dylib"]
#![allow(non_camel_case_types)]
/* automatically generated by rust-bindgen */
extern crate libc;

pub type __int128_t = ::libc::c_void;
pub type __uint128_t = ::libc::c_void;
pub type __builtin_va_list = [__va_list_tag; 1us];
pub type __va_list_tag = Struct___va_list_tag;
#[repr(C)]
pub struct Struct___va_list_tag {
    pub gp_offset: ::libc::c_uint,
    pub fp_offset: ::libc::c_uint,
    pub overflow_arg_area: *mut ::libc::c_void,
    pub reg_save_area: *mut ::libc::c_void,
}
#[link(name = "avdevice")]
extern "C" {
    pub fn avdevice_version() -> ::libc::c_uint;
    pub fn avdevice_configuration() -> *const ::libc::c_char;
    pub fn avdevice_license() -> *const ::libc::c_char;
    pub fn avdevice_register_all();
}

pub fn version() -> usize {
    unsafe {
        avdevice_version() as usize
    }
}
pub fn license() -> &'static str {
    unsafe {
        std::str::from_c_str(avdevice_license())
    }
}
