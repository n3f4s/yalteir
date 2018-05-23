extern crate nix;
extern crate void;

pub fn chdir(path: std::ffi::CString) -> Result<void::Void, nix::errno::Errno> {
    if chdir(path) == -1 {
        Err(nix::errno::from_i32(nix::errno::errno()))
    }
    Ok(void::Void{})
}
