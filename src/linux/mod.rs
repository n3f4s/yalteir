// linux
// Linux C API not available in nix crate

extern crate nix;
extern crate void;
extern crate libc;

// FIXME libc::getenv

use std;

pub fn chdir(path: std::ffi::CString) -> Result<(), nix::errno::Errno> {
    unsafe {
        if libc::chdir(path.as_ptr()) == -1 {
            return Err(nix::errno::from_i32(nix::errno::errno()))
        }
        Ok(())
    }
}

pub fn getenv(var: std::ffi::CString) -> Option<std::ffi::CString> {
    unsafe {
        let ret = libc::getenv(var.as_ptr());
        if ret.is_null() {
            return None
        }
        Some(std::ffi::CString::from_raw(ret))
    }
}

