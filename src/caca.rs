#[macro_use] extern crate bitflags;
extern crate errno;
extern crate libc;
extern crate caca_sys as caca;

pub mod dither;
pub mod event;
pub mod keyboard;
pub mod primitives;

pub use keyboard::Key;
pub use dither::CacaDither;

pub use event::{
    Event,
        EVENT_NONE,
        EVENT_KEY_PRESS,
        EVENT_KEY_RELEASE,
        EVENT_MOUSE_PRESS,
        EVENT_MOUSE_RELEASE,
        EVENT_MOUSE_MOTION,
        EVENT_RESIZE,
        EVENT_QUIT,
        EVENT_ANY,
};

use std::default::Default;
use std::ffi::{CStr, CString};
use std::ptr::null_mut;

use caca::*;
use errno::errno;

pub enum AnsiColor {
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

impl AnsiColor {
    fn as_byte(&self) -> u8 {
        let color = match *self {
            AnsiColor::Black        => caca::CACA_BLACK,
            AnsiColor::Blue         => caca::CACA_BLUE,
            AnsiColor::Green        => caca::CACA_GREEN,
            AnsiColor::Cyan         => caca::CACA_CYAN,
            AnsiColor::Red          => caca::CACA_RED,
            AnsiColor::Magenta      => caca::CACA_MAGENTA,
            AnsiColor::Brown        => caca::CACA_BROWN,
            AnsiColor::LightGray    => caca::CACA_LIGHTGRAY,
            AnsiColor::DarkGray     => caca::CACA_DARKGRAY,
            AnsiColor::LightBlue    => caca::CACA_LIGHTBLUE,
            AnsiColor::LightGreen   => caca::CACA_LIGHTGREEN,
            AnsiColor::LightCyan    => caca::CACA_LIGHTCYAN,
            AnsiColor::LightRed     => caca::CACA_LIGHTRED,
            AnsiColor::LightMagenta => caca::CACA_LIGHTMAGENTA,
            AnsiColor::Yellow       => caca::CACA_YELLOW,
            AnsiColor::White        => caca::CACA_WHITE,
            AnsiColor::Default      => caca::CACA_DEFAULT,
            AnsiColor::Transparent  => caca::CACA_TRANSPARENT,
        };
        color as u8
    }
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

pub enum Visibility {
    Hide,
    Show,
}

impl Visibility {
    fn as_flag(&self) -> i32 {
        match *self {
            Visibility::Hide => 0,
            Visibility::Show => 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Driver {
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

impl Driver {
    fn from_cstr(cs: &CStr) -> Driver {
        let string = cs.to_str().unwrap();
        match string {
            "null"    => Driver::Null,
            "raw"     => Driver::Raw,
            "cocoa"   => Driver::Cocoa,
            "conio"   => Driver::Conio,
            "gl"      => Driver::GL,
            "ncurses" => Driver::NCurses,
            "slang"   => Driver::SLang,
            "vga"     => Driver::VGA,
            "win32"   => Driver::Win32,
            "x11"     => Driver::X11,
            _         => Driver::Unknown,
        }
    }
    fn to_cstring(&self) -> CString {
        let driver_name = match *self {
            Driver::Unknown |
            Driver::Null    => "null",
            Driver::Raw     => "raw",
            Driver::Cocoa   => "cocoa",
            Driver::Conio   => "conio",
            Driver::GL      => "gl",
            Driver::NCurses => "ncurses",
            Driver::SLang   => "slang",
            Driver::VGA     => "vga",
            Driver::Win32   => "win32",
            Driver::X11     => "x11",
        };
        CString::new(driver_name).unwrap()
    }
}

pub struct CacaDisplay {
    display: *mut CacaDisplayRaw,
}

#[derive(Clone, Copy)]
pub struct InitOptions<'b, 'a: 'b> {
    pub canvas: Option<&'b CacaCanvas<'a>>,
    pub driver: Option<Driver>,
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
    InvalidMaskSize,
    CanvasInUse,
    InvalidRefreshDelay,
    WindowTitleUnsupported,
    MousePointerUnsupported,
    MouseCursorUnsupported,
    InvalidDitherParams,
    InvalidBrightness,
    InvalidGamma,
    InvalidContrast,
    Unknown,
}

pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

pub type CacaResult = Result<(), CacaError>;

impl CacaDisplay {
    pub fn new(opts: InitOptions) -> Result<Self, CacaError> {
        let canvas_ptr = match opts.canvas {
            Some(canvas_) => unsafe { canvas_.as_mut_ptr() },
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

        // FIXME: The errno is always EINVAL here except with width and height 0!
        match errno().0 {
            libc::ENOMEM => Err(CacaError::NotEnoughMemory),
            //libc::ENODEV => Err(CacaError::FailedToOpenGraphicsDevice),
            _      => Ok(CacaDisplay {
                display: display,
            }),
        }
    }

    pub fn display_driver(&self) -> Driver {
        unsafe {
            let raw_str = caca_get_display_driver(self.display);
            let driver_name = CStr::from_ptr(raw_str);
            Driver::from_cstr(driver_name)
        }
    }

    pub fn set_display_driver(&mut self, driver: Driver) -> CacaResult {
        let driver_str = driver.to_cstring();
        let result = unsafe { caca_set_display_driver(self.display, driver_str.as_ptr()) };
        if result == -1 {
            Err(CacaError::Unknown)
        } else {
            Ok(())
        }
    }

    #[cfg(never)]
    pub fn driver_list() -> Vec<Driver> {
        let c_list = unsafe { caca_get_display_driver_list() };
        c_list.map(|str| str.as_ptr())
            .map(|ptr| CStr::from_ptr(ptr).to_str().unwrap())
    }

    pub fn canvas(&self) -> CacaCanvas {
        let mut displays = Vec::new();
        displays.push(self);
        CacaCanvas {
            canvas: unsafe { caca_get_canvas(self.display) },
            displays: displays,
        }
    }

    pub fn refresh(&mut self) {
        unsafe { caca_refresh_display(self.display) };
    }

    pub fn display_time(&self) -> i32 {
        unsafe { caca_get_display_time(self.display) }
    }

    pub fn width(&self) -> i32 {
        unsafe { caca_get_display_width(self.display) }
    }

    pub fn height(&self) -> i32 {
        unsafe { caca_get_display_height(self.display) }
    }

    pub fn set_display_title(&mut self, title: &str) -> CacaResult {
        let title_cstring = CString::new(title).unwrap();
        unsafe { caca_set_display_title(self.display, title_cstring.as_ptr()) };
        match errno().0 {
            libc::ENOSYS => Err(CacaError::WindowTitleUnsupported),
            _            => Ok(())
        }
    }

    pub fn set_mouse_visibility(&mut self, vis: Visibility) -> CacaResult {
        unsafe { caca_set_mouse(self.display, vis.as_flag()) };
        match errno().0 {
            libc::ENOSYS => Err(CacaError::MousePointerUnsupported),
            _            => Ok(())
        }
    }

    pub fn set_cursor_visibility(&mut self, vis: Visibility) -> CacaResult {
        unsafe { caca_set_mouse(self.display, vis.as_flag()) };
        match errno().0 {
            libc::ENOSYS => Err(CacaError::MouseCursorUnsupported),
            _            => Ok(())
        }
    }

    pub fn set_display_time(&mut self, usec: i32) -> CacaResult {
        unsafe { caca_set_display_time(self.display, usec) };
        match errno().0 {
            libc::EINVAL => Err(CacaError::InvalidRefreshDelay),
            _      => Ok(())
        }
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
    pub fn new(width: i32, height: i32) -> Result<Self, CacaError> {
        // FIXME: kludge, this should be detected by libcaca
        if width < 0 || height < 0 {
            return Err(CacaError::InvalidSize);
        }

        let canvas = unsafe {
            caca_create_canvas(width, height)
        };

        // FIXME: The errno is always EINVAL here except with width and height 0!
        match errno().0 {
            libc::ENOMEM => Err(CacaError::NotEnoughMemory),
            //libc::EINVAL => Err(CacaError::InvalidSize),
            _      => Ok(CacaCanvas {
                canvas: canvas,
                displays: Vec::new(),
            }),
        }
    }

    pub unsafe fn as_mut_ptr(&self) -> *mut CacaCanvasRaw {
        self.canvas
    }

    pub fn set_color_ansi(&mut self, fg: AnsiColor, bg: AnsiColor) {
        unsafe { caca_set_color_ansi(self.canvas, fg.as_byte(), bg.as_byte()) };
    }

    pub fn put_char(&mut self, x: i32, y: i32, c: char) -> usize {
        unsafe { caca_put_char(self.canvas, x, y, c as u32) as usize }
    }

    pub fn put_str(&mut self, x: i32, y: i32, s: &str) -> usize {
        let cstring = CString::new(s).unwrap();
        unsafe { caca_put_str(self.canvas, x, y, cstring.as_ptr()) as usize}
    }

    pub fn clear(&mut self) {
        unsafe { caca_clear_canvas(self.canvas) };
    }

    pub fn set_handle(&mut self, x: i32, y: i32) {
        unsafe { caca_set_canvas_handle(self.canvas, x, y) };
    }

    pub fn handle_x(&self) -> i32 {
        unsafe { caca_get_canvas_handle_x(self.canvas) }
    }

    pub fn handle_y(&self) -> i32 {
        unsafe { caca_get_canvas_handle_y(self.canvas) }
    }

    pub fn blit(&mut self, x: i32, y: i32, source: &CacaCanvas, mask: &CacaCanvas) -> CacaResult {
        unsafe { caca_blit(self.canvas, x, y, source.canvas, mask.canvas) };
        match errno().0 {
            libc::EINVAL => Err(CacaError::InvalidMaskSize),
            _            => Ok(())
        }
    }

    pub fn set_boundaries(&mut self, x: i32, y: i32, w: i32, h: i32) -> CacaResult {
        unsafe { caca_set_canvas_boundaries(self.canvas, x, y, w, h) };
        match errno().0 {
            libc::EINVAL => Err(CacaError::InvalidSize),
            libc::EBUSY  => Err(CacaError::CanvasInUse),
            libc::ENOMEM => Err(CacaError::NotEnoughMemory),
            _            => Ok(())
        }
    }

    pub fn set_size(&mut self, width: i32, height: i32) -> CacaResult {
        // FIXME: kludge, this should be detected by libcaca
        if width < 0 || height < 0 {
            return Err(CacaError::InvalidSize);
        }

        unsafe { caca_set_canvas_size(self.canvas, width, height); }
        println!("{} {} {:?}", width, height, errno().0);
        // FIXME: The errno is always EINVAL here except with width and height 0!
        match errno().0 {
            //libc::EINVAL => Err(CacaError::InvalidSize),
            libc::EBUSY  => Err(CacaError::CanvasInUse),
            libc::ENOMEM => Err(CacaError::NotEnoughMemory),
            _      => Ok(()),
        }
    }

    pub fn width(&self) -> i32 {
        unsafe { caca_get_canvas_width(self.canvas) as i32 }
    }

    pub fn height(&self) -> i32 {
        unsafe { caca_get_canvas_height(self.canvas) as i32 }
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
