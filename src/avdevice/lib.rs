#[crate_id = "avdevice#54"];
#[license = "MIT"];
#[crate_type = "dylib"];
/* automatically generated by rust-bindgen */
use std::libc::{c_void,c_uint,c_schar};

pub type __int128_t = c_void;
pub type __uint128_t = c_void;
pub type __builtin_va_list = [__va_list_tag, ..1u];
pub type __va_list_tag = Struct___va_list_tag;
pub struct Struct___va_list_tag {
    gp_offset: c_uint,
    fp_offset: c_uint,
    overflow_arg_area: *mut c_void,
    reg_save_area: *mut c_void,
}
#[link(name = "avdevice")]
extern "C" {
    pub fn avdevice_version() -> c_uint;
    pub fn avdevice_configuration() -> *c_schar;
    pub fn avdevice_license() -> *c_schar;
    pub fn avdevice_register_all();
}

pub fn version() -> uint{
    unsafe {
        avdevice_version() as uint
    }
}
pub fn license() -> ~str {
    unsafe {
        std::str::raw::from_c_str(avdevice_license())
    }
}
