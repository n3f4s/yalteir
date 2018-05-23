
use x11::x11_h;
use x11::x11_util_h;

// FIXME placeholder
pub type Pixmap = x11_h::Pixmap;
pub type XSizeHints = x11_util_h::XSizeHints;
pub type XGCValues = x11_h::XGCValues;
pub type XEvent = x11_h::XEvent;

pub struct GC<'a> {
    pub(crate) ptr: x11_h::GC,
    pub(crate) display: &'a mut Display,
}

pub type Color = u64;

pub struct Display {
    pub(crate) ptr: *mut x11_h::Display,
}

pub struct Visual {
    pub(crate) ptr: *mut x11_h::Visual,
}

pub trait Drawable {
    fn get_ptr(&self) -> u64;
}
// FIXME thread safety
pub struct Window<'a> {
    pub(crate) ptr: x11_h::Window,
    pub(crate) display: &'a mut Display,
}

pub struct Screen<'a> {
    pub(crate) id: i32,
    pub(crate) display: &'a mut Display,
}

impl<'a> Drawable for Window<'a> {
    fn get_ptr(&self) -> u64 {
        self.ptr
    }
}

impl<'a> Drop for GC<'a> {
    fn drop(& mut self) {
        unsafe {
            x11_h::XFreeGC(self.display.ptr, self.ptr);
        }
    }
}

impl<'a> Drop for Window<'a> {
    fn drop(& mut self) {
        unsafe {
            x11_h::XDestroyWindow(self.display.ptr, self.ptr);
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            x11_h::XCloseDisplay(self.ptr);
        }
    }
}
