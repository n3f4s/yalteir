extern crate nix;
extern crate libc;
extern crate core;

use std::error::Error;
use libc::{STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO, EOF};
use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::os::unix::process::CommandExt;

use nix::sys::select::FdSet;

mod linux;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
mod x11;

fn term_process(master: RawFd, child: nix::unistd::Pid) {
    let mut run = true;
    loop {
        if ! run {
            // kill the shell
            break;
        }
        let mut read_fd = FdSet::new();
        read_fd.insert(master);
        read_fd.insert(STDIN_FILENO);
        let mut write_fd = FdSet::new();
        let mut err_fd = FdSet::new();
        let _ = nix::sys::select::select(master+1, Some(& mut read_fd),
                                         Some(& mut write_fd), Some(& mut err_fd), None);
        // talk_to_shell
        if read_fd.contains(master) {
            let mut buf: [u8; 1] = [0];
            match nix::unistd::read(master, & mut buf) {
                Ok(_) => {
                    //println!("\nchar: {:?}", buf);
                    if buf[0] == 4 || buf[0] == 3{
                        println!("\nchar: {:?}", buf);
                        run = false
                    }
                    match nix::unistd::write(STDOUT_FILENO, &buf) {
                        Ok(_) => {
                        },
                        Err(_) => run = false
                    };
                },
                Err(_) => run = false
            };
        }
        // talk_to_stdin
        if read_fd.contains(STDIN_FILENO) {
            let mut buf: [u8; 1] = [0];
            match nix::unistd::read(STDIN_FILENO, & mut buf) {
                Ok(0) => {
                    buf = [EOF as u8];
                    let _ = nix::unistd::write(master, &buf);
                },
                Ok(_) => {
                    // FIXME terminal dependant ???
                    if buf[0] == 4 || buf[0] == 3{
                        println!("\nchar: {:?}", buf);
                        run = false
                    }
                    match nix::unistd::write(master, &buf) {
                        Ok(_) => {
                        },
                        Err(_) => run = false
                    };
                },
                Err(_) => run = false
            };
        }
    }
    let _ = nix::sys::signal::kill(child, nix::sys::signal::Signal::SIGKILL);
}

fn main() {

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
        libc::tcgetattr(STDIN_FILENO, &mut term as *mut libc::termios);
        term.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON);
        libc::tcsetattr(STDIN_FILENO, libc::TCSAFLUSH, &mut term as *mut libc::termios);
    }

    // slave -> term
    // master -> shell
    let file_descriptors = match nix::pty::openpty(None, None) {
        Ok(fd) => fd,
        Err(err) => match err {
            nix::Error::Sys(errno) => panic!("{}", errno.description()),
            _ => panic!("Strange error !")
        }
    };

    let slave = file_descriptors.slave;

    // FIXME get shell from getenv
    let child = std::process::Command::new("/usr/bin/bash").before_exec(move || -> Result<(), std::io::Error>{
        let _ = match nix::unistd::setsid() {
            Ok(gid) => gid,
            Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err))
        };
        //println!("Shell: dup2({}, {})", slave, STDIN_FILENO);
        let _ = match nix::unistd::dup2(slave, STDIN_FILENO) {
            Ok(fd) => fd,
            Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err))
        };
        //println!("Shell: dup2({}, {})", slave, STDERR_FILENO);
        let _ = match nix::unistd::dup2(slave, STDERR_FILENO) {
            Ok(fd) => fd,
            Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err))
        };
        //println!("Shell: dup2({}, {})", slave, STDOUT_FILENO);
        let _ = match nix::unistd::dup2(slave, STDOUT_FILENO) {
            Ok(fd) => fd,
            Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err))
        };
        match linux::chdir(CString::new(
            linux::getenv(
                CString::new("HOME").unwrap()
            ).unwrap()
        ).unwrap()) {
            Ok(_) => {},
            Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, nix::Error::Sys(err)))
        }
        return Ok(())
    }).spawn();
    term_process(file_descriptors.master, nix::unistd::Pid::from_raw(child.unwrap().id() as i32));
}
