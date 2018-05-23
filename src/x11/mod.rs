
mod x11_h;
mod x11_util_h;
mod types;
mod c_api;

use x11::types::{
    XSizeHints,
    Display,
    Window,
    Color,
    GC,
    Screen,
    Visual,
    Drawable,
    XGCValues,
    XEvent,
    Pixmap};
use x11::c_api::*;

// FIXME error handling
// FIXME DRY
// FIXME const correctness
// FIXME implement methods
// FIXME better API
// FIXME replace screen: i32 by screen: Screen
// FIXME freeing function as no-op

// FIXME make 2 import: one with c-like API, other with OOP-like API


impl<'a> Display {
    pub fn XDefaultScreen(&mut self) -> Screen {
        Screen {
            id: XDefaultScreen(self),
            display: self,
        }
    }
}

impl<'a> Screen<'a> {
    pub fn XDefaultVisual(&mut self) -> Visual {
        XDefaultVisual(self.display, self.id)
    }
    pub fn BlackPixel(&mut self) -> Color {
        BlackPixel(self.display, self.id)
    }
    pub fn WhitePixel(&mut self) -> Color {
        WhitePixel(self.display, self.id)
    }
}

impl<'a> Window<'a> {
    pub fn XCreateSimpleWindow(&mut self, win_name: ::std::ffi::CString,
                               pos: (i32, i32), dim: (u32, u32),
                               border_width: u32, border_color: Color,
                               bg_color: Color) -> Window<'a> {
        let (pos_x, pos_y) = pos;
        let (width, height) = dim;
        XCreateSimpleWindow(self.display, self, pos_x, pos_y, width, height,
                            border_width, border_color, bg_color)
    }
    pub fn XSetStandardProperties(&mut self,
                                      win_name: ::std::ffi::CString,
                                      icon_name: ::std::ffi::CString, icon: Pixmap,
                                      args: &mut ::std::vec::Vec<::std::ffi::CString>,
                                      hints: &mut XSizeHints) {
        XSetStandardProperties(self.display, self, win_name,
                                   icon_name, icon, args, hints)
    }
    pub fn XSelectInput(&mut self, mask: i64) {
        XSelectInput(self.display, self, mask)
    }
}


// TODO : better API from there
pub fn XCreateGC<'a, T: Drawable>(dis: &'a mut Display, win: &'a mut T,
                                  mask: u64, values: &mut XGCValues) -> GC<'a> {
    GC {
        ptr: unsafe { x11_h::XCreateGC(dis.ptr, win.get_ptr(), mask, values as *mut XGCValues) },
        display: dis,
    }
}

pub fn XSetBackground<'a>(dis: &'a mut Display, gc: &'a GC, color: Color) {
    unsafe {
        x11_h::XSetBackground(dis.ptr, gc.ptr, color);
    }
}
pub fn XSetForeground<'a>(dis: &'a mut Display, gc: &'a GC, color: Color) {
    unsafe {
        x11_h::XSetForeground(dis.ptr, gc.ptr, color);
    }
}

pub fn XClearWindow<'a>(dis: &'a mut Display, win: &'a mut Window) {
    unsafe {
        x11_h::XClearWindow(dis.ptr, win.ptr);
    }
}
pub fn XMapRaised<'a>(dis: &'a mut Display, win: &'a mut Window) {
    unsafe {
        x11_h::XMapRaised(dis.ptr, win.ptr);
    }
}

pub fn XNextEvent(dis: &mut Display) -> XEvent {
    unsafe {
        let mut event: x11_h::XEvent = ::std::mem::uninitialized();
        x11_h::XNextEvent(dis.ptr, &mut event as * mut x11_h::XEvent);
        event
    }
}
