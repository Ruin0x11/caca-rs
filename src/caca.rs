#[macro_use] extern crate bitflags;
extern crate errno;
extern crate libc;
extern crate num;
extern crate caca_sys as caca;

use std::default::Default;
use std::ffi::{CStr, CString};
use std::ptr::null_mut;

use caca::*;
use errno::errno;

pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White,
    Default,
    Transparent,
}

bitflags! {
    #[repr(C)]
    flags Style: u32 {
        const CACA_BOLD = caca::CACA_BOLD,
        const CACA_ITALICS = caca::CACA_ITALICS,
        const CACA_UNDERLINE = caca::CACA_UNDERLINE,
        const CACA_BLINK = caca::CACA_BLINK,
    }
}

pub enum Event {
    None,
    // KeyPress(Key),
    // KeyRelease(Key),
    // MousePress(MouseButton),
    // MouseRelease(MouseButton),
    MouseMotion(i32, i32),
    Resize(i32, i32),
    Quit,
    Any,
    Unknown(isize),
}

#[derive(Clone, Copy, Debug)]
pub enum DriverType {
    Null,
    Raw,
    Cocoa,
    Conio,
    GL,
    NCurses,
    SLang,
    VGA,
    Win32,
    X11,
    Unknown,
}

impl DriverType {
    fn from_str(s: &str) -> DriverType {
        match s {
            "null"    => DriverType::Null,
            "raw"     => DriverType::Raw,
            "cocoa"   => DriverType::Cocoa,
            "conio"   => DriverType::Conio,
            "gl"      => DriverType::GL,
            "ncurses" => DriverType::NCurses,
            "slang"   => DriverType::SLang,
            "vga"     => DriverType::VGA,
            "win32"   => DriverType::Win32,
            "x11"     => DriverType::X11,
            _         => DriverType::Unknown,
        }
    }
    fn to_cstring(&self) -> CString {
        let driver_name = match *self {
            DriverType::Unknown |
            DriverType::Null    => "null",
            DriverType::Raw     => "raw",
            DriverType::Cocoa   => "cocoa",
            DriverType::Conio   => "conio",
            DriverType::GL      => "gl",
            DriverType::NCurses => "ncurses",
            DriverType::SLang   => "slang",
            DriverType::VGA     => "vga",
            DriverType::Win32   => "win32",
            DriverType::X11     => "x11",
        };
        CString::new(driver_name).unwrap()
    }
}

// type EventResult = Result<Event, EventError>;

#[cfg(never)]
fn unpack_event(event_type: c_uint, ev: &CacaEventRaw) -> EventResult {
    match event_type {
        caca::CACA_EVENT_NONE => Ok(Event::None),
        caca::CACA_EVENT_KEY_PRESS => Ok(
            Event::KeyEvent()
        )
    }
}

pub struct CacaDisplay {
    driver: DriverType,
    display: *mut CacaDisplayRaw,
}

#[derive(Clone, Copy)]
pub struct InitOptions<'b, 'a: 'b> {
    pub canvas: Option<&'b CacaCanvas<'a>>,
    pub driver: Option<DriverType>,
    pub buffer_stderr: bool,
}

impl<'b, 'a> Default for InitOptions<'b, 'a> {
    fn default() -> Self {
        InitOptions {
            canvas: None,
            driver: None,
            buffer_stderr: false,
        }
    }
}

#[derive(Debug)]
pub enum CacaError {
    NotEnoughMemory,
    FailedToOpenGraphicsDevice,
    InvalidSize,
    CanvasInUse,
}

impl CacaDisplay {
    pub fn new(opts: InitOptions) -> Result<CacaDisplay, CacaError> {
        let canvas_ptr = match opts.canvas {
            Some(canvas_) => unsafe { canvas_.get_mut_ptr() },
            None          => null_mut(),
        };

        let display = unsafe {
            match opts.driver {
                Some(driver_)     => {
                    let driver_name = driver_.to_cstring();
                    caca_create_display_with_driver(canvas_ptr, driver_name.as_ptr())
                },
                None => caca_create_display(canvas_ptr),
            }
        };

        // since the CacaDisplay hasn't been created yet, we can't use an
        // instance method for getting the driver that was actually initialized.
        let driver = get_driver_i(display);

        match errno().0 {
            libc::ENOMEM => Err(CacaError::NotEnoughMemory),
            libc::ENODEV => Err(CacaError::FailedToOpenGraphicsDevice),
            _      => Ok(CacaDisplay {
                driver: driver,
                display: display,
            }),
        }
    }

    pub fn get_driver(&self) -> DriverType {
        self.driver
    }
}

fn get_driver_i(display: *mut CacaDisplayRaw) -> DriverType {
    unsafe {
        let raw_str = caca_get_display_driver(display);
        let driver_name = CStr::from_ptr(raw_str);
        DriverType::from_str(driver_name.to_str().unwrap())
    }
}

impl Drop for CacaDisplay {
    fn drop(&mut self) {
        unsafe {
            caca_free_display(self.display);
        }
    }
}

pub struct CacaCanvas<'a> {
    canvas: *mut CacaCanvasRaw,
    displays: Vec<&'a CacaDisplay>,
}

impl<'a> CacaCanvas<'a> {
    pub fn new(width: i32, height: i32) -> Result<CacaCanvas<'a>, CacaError> {
        let canvas = unsafe {
            caca_create_canvas(width, height)
        };

        match errno().0 {
            libc::ENOMEM => Err(CacaError::NotEnoughMemory),
            libc::EINVAL => Err(CacaError::InvalidSize),
            _      => Ok(CacaCanvas {
                canvas: canvas,
                displays: Vec::new(),
            }),
        }
    }

    pub unsafe fn get_mut_ptr(&self) -> *mut CacaCanvasRaw {
        self.canvas
    }
}

impl<'a> CacaCanvas<'a> {
    fn set_size(&mut self, width: i32, height: i32) -> Result<(), CacaError>{
        unsafe { caca_set_canvas_size(self.canvas, width, height); }
        match errno().0 {
            libc::EINVAL => Err(CacaError::InvalidSize),
            libc::EBUSY  => Err(CacaError::CanvasInUse),
            libc::ENOMEM => Err(CacaError::NotEnoughMemory),
            _      => Ok(()),
        }
    }

    fn width(&self) -> usize {
        unsafe { caca_get_canvas_width(self.canvas) as usize }
    }

    fn height(&self) -> usize {
        unsafe { caca_get_canvas_height(self.canvas) as usize }
    }
}

impl<'a> Drop for CacaCanvas<'a> {
    fn drop(&mut self) {
        unsafe {
            caca_free_canvas(self.canvas);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_canvas_and_display() -> (CacaCanvas<'static>, CacaDisplay) {
        let canvas = CacaCanvas::new(100, 100);
        assert!(canvas.is_ok(), "{:?}", canvas.err());
        let canvas_ok = canvas.unwrap();
        let display = CacaDisplay::new(InitOptions{canvas: Some(&canvas_ok),
                                                   driver: Some(DriverType::Null),
                                                   ..InitOptions::default()});
        assert!(display.is_ok(), "{:?}, display.err()");
        (canvas_ok, display.unwrap())
    }

    #[test]
    fn test_resize_canvas() {
        let (mut canvas, _) = get_canvas_and_display();

        let result = canvas.set_size(50, 50);
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(canvas.width(), 50);
        assert_eq!(canvas.height(), 50);

        let result = canvas.set_size(100, 75);
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(canvas.width(), 100);
        assert_eq!(canvas.height(), 75);

        let result = canvas.set_size(-100, -100);
        assert!(result.is_err());
    }
}
