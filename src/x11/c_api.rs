
use x11::x11_h;
use x11::x11_util_h;

use x11::types::{Pixmap,
                 XSizeHints,
                 Display,
                 Visual,
                 Color,
                 Window};

use std::ffi::CString;

pub fn XOpenDisplay(arg1: CString) -> Display {
    Display {
        ptr: unsafe { x11_h::XOpenDisplay(arg1.as_ptr())}
    }
}

pub fn XDefaultScreen(display: &mut Display) -> i32 {
    unsafe {
        x11_h::XDefaultScreen(display.ptr)
    }
}
pub fn XDefaultVisual(display: &mut Display, screen: i32) -> Visual {
    Visual {
        ptr: unsafe { x11_h::XDefaultVisual(display.ptr, screen)}
    }
}
pub fn BlackPixel(display: &mut Display, screen: i32) -> Color {
    unsafe {
        x11_h::XBlackPixel(display.ptr, screen)
    }
}
pub fn WhitePixel(display: &mut Display, screen: i32) -> Color {
    unsafe {
        x11_h::XWhitePixel(display.ptr, screen)
    }
}

pub fn XCreateSimpleWindow<'a>(dis: &'a mut Display, parent: &'a mut Window,
                           pos_x: i32, pos_y: i32,
                           width: u32, height: u32,
                           border_width: u32, border_color: Color,
                           bg_color: Color) -> Window<'a> {
    Window {
        ptr: unsafe { x11_h::XCreateSimpleWindow(dis.ptr, parent.ptr,
                                                 pos_x, pos_y,
                                                 width, height,
                                                 border_width, border_color,
                                                 bg_color) },
        display: dis,
    }
}

pub fn XSetStandardProperties<'a>(dis: &'a mut Display, win: &'a mut Window,
                                  win_name: ::std::ffi::CString,
                                  icon_name: ::std::ffi::CString, icon: Pixmap,
                                  args: &mut ::std::vec::Vec<::std::ffi::CString>,
                                  hints: &mut XSizeHints) {
    let tmp: ::std::vec::Vec<*mut i8> = args.iter().map(|x| x.as_ptr() as *mut i8).collect();
    unsafe {
        x11_util_h::XSetStandardProperties(dis.ptr as *mut x11_util_h::Display, win.ptr,
                                           win_name.as_ptr(), icon_name.as_ptr(), icon,
                                           tmp.as_ptr() as *mut *mut i8, args.len() as i32,
                                           hints as *mut XSizeHints);
    }
}
pub fn XSelectInput<'a>(dis: &'a mut Display, win: &'a mut Window, mask: i64) {
    unsafe {
        x11_h::XSelectInput(dis.ptr, win.ptr, mask);
    }
}
