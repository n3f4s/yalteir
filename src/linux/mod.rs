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

pub struct Termios {
    pub term: libc::termios,
}

impl Termios {
    pub fn new(fd: i32) -> Termios {
        let mut term: libc::termios = libc::termios{
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0
        };
        unsafe {
            libc::tcgetattr(fd, &mut term as *mut libc::termios);
        }
        Termios {
            term: term
        }
    }

    pub fn tcsetattr(&mut self, fd: i32, opt_act: i32) {
        unsafe {
            libc::tcsetattr(fd, opt_act, &mut self.term as *mut libc::termios);
        }
    }
}
