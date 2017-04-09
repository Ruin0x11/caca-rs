#[macro_use] extern crate bitflags;
extern crate errno;
extern crate libc;
extern crate caca_sys as caca;

pub mod dither;
pub mod event;
pub mod keyboard;
pub mod primitives;

pub use keyboard::Key;
pub use dither::Dither;

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
use std::marker::PhantomData;
use std::ptr::null_mut;
use std::time::Duration;

use caca::*;
use errno::errno;
use libc::c_int;

#[derive(Clone, Copy, Debug)]
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

    pub fn from_byte(byte: u8) -> Self {
        match byte as u32 {
            caca::CACA_BLACK                =>     AnsiColor::Black,
            caca::CACA_BLUE                 =>     AnsiColor::Blue,
            caca::CACA_GREEN                =>     AnsiColor::Green,
            caca::CACA_CYAN                 =>     AnsiColor::Cyan,
            caca::CACA_RED                  =>     AnsiColor::Red,
            caca::CACA_MAGENTA              =>     AnsiColor::Magenta,
            caca::CACA_BROWN                =>     AnsiColor::Brown,
            caca::CACA_LIGHTGRAY            =>     AnsiColor::LightGray,
            caca::CACA_DARKGRAY             =>     AnsiColor::DarkGray,
            caca::CACA_LIGHTBLUE            =>     AnsiColor::LightBlue,
            caca::CACA_LIGHTGREEN           =>     AnsiColor::LightGreen,
            caca::CACA_LIGHTCYAN            =>     AnsiColor::LightCyan,
            caca::CACA_LIGHTRED             =>     AnsiColor::LightRed,
            caca::CACA_LIGHTMAGENTA         =>     AnsiColor::LightMagenta,
            caca::CACA_YELLOW               =>     AnsiColor::Yellow,
            caca::CACA_WHITE                =>     AnsiColor::White,
            caca::CACA_DEFAULT              =>     AnsiColor::Default,
            caca::CACA_TRANSPARENT          =>     AnsiColor::Transparent,
            _ => panic!("Invalid byte to ansi color conversion!"),
        }
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

pub struct Display {
    display: *mut CacaDisplayRaw,
    _phantom: PhantomData<*mut ()>,
}

#[derive(Clone, Copy)]
pub struct InitOptions<'b, 'a: 'b> {
    pub canvas: Option<&'b Canvas<'a>>,
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
    InvalidFrameIndex,
    Unknown(i32),
}

pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

pub type CacaResult = Result<(), CacaError>;

impl Display {
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

        if display.is_null() {
            let errno = errno().0;
            match errno {
                libc::EINVAL => Err(CacaError::InvalidSize),
                libc::ENOMEM => Err(CacaError::NotEnoughMemory),
                libc::ENODEV => Err(CacaError::FailedToOpenGraphicsDevice),
                _            => Err(CacaError::Unknown(errno)),
            }
        }
        else {
            Ok(Display {
                display: display,
                _phantom: PhantomData,
            })
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
            Err(CacaError::Unknown(errno().0))
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

    pub fn canvas(&self) -> Canvas {
        let mut displays = Vec::new();
        displays.push(self);
        Canvas {
            canvas: unsafe { caca_get_canvas(self.display) },
            displays: displays,
            _phantom: PhantomData,
        }
    }

    pub fn refresh(&mut self) {
        unsafe { caca_refresh_display(self.display) };
    }

    pub fn calculated_display_time(&self) -> i32 {
        unsafe { caca_get_display_time(self.display) }
    }

    pub fn set_display_time(&mut self, usecs: Duration) -> CacaResult {
        let display_time = (usecs.as_secs() * 1000 + usecs.subsec_nanos() as u64 / 1000000) as c_int;
        let result = unsafe { caca_set_display_time(self.display, display_time) };
        if result == 0 {
            Ok(())
        } else {
            let errno = errno().0;
            match errno {
                libc::EINVAL => Err(CacaError::InvalidRefreshDelay),
                _            => Err(CacaError::Unknown(errno)),
            }
        }
    }

    pub fn width(&self) -> i32 {
        unsafe { caca_get_display_width(self.display) }
    }

    pub fn height(&self) -> i32 {
        unsafe { caca_get_display_height(self.display) }
    }

    pub fn set_display_title(&mut self, title: &str) -> CacaResult {
        let title_cstring = CString::new(title).unwrap();
        let result = unsafe { caca_set_display_title(self.display, title_cstring.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            let errno = errno().0;
            match errno {
                libc::ENOSYS => Err(CacaError::WindowTitleUnsupported),
                _            => Err(CacaError::Unknown(errno)),
            }
        }
    }

    pub fn set_mouse_visibility(&mut self, vis: Visibility) -> CacaResult {
        let result = unsafe { caca_set_mouse(self.display, vis.as_flag()) };
        if result == 0 {
            Ok(())
        } else {
            let errno = errno().0;
            match errno {
                libc::ENOSYS => Err(CacaError::MousePointerUnsupported),
                _            => Err(CacaError::Unknown(errno)),
            }
        }
    }

    pub fn set_cursor_visibility(&mut self, vis: Visibility) -> CacaResult {
        let result = unsafe { caca_set_mouse(self.display, vis.as_flag()) };
        if result == 0 {
            Ok(())
        } else {
            let errno = errno().0;
            match errno {
                libc::ENOSYS => Err(CacaError::MouseCursorUnsupported),
                _            => Err(CacaError::Unknown(errno)),
            }
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            caca_free_display(self.display);
        }
    }
}

pub struct Canvas<'a> {
    canvas: *mut CacaCanvasRaw,
    displays: Vec<&'a Display>,
    _phantom: PhantomData<*mut ()>,
}

impl<'a> Canvas<'a> {
    pub fn new(width: i32, height: i32) -> Result<Self, CacaError> {
        // FIXME: kludge, this should be detected by libcaca
        if width < 0 || height < 0 {
            return Err(CacaError::InvalidSize);
        }

        let canvas = unsafe {
            caca_create_canvas(width, height)
        };

        if canvas.is_null() {
            match errno().0 {
                libc::ENOMEM => Err(CacaError::NotEnoughMemory),
                libc::EINVAL => Err(CacaError::InvalidSize),
                _            => Err(CacaError::Unknown(errno().0)),
            }
        } else {
            Ok(Canvas {
                canvas: canvas,
                displays: Vec::new(),
                _phantom: PhantomData,
            })
        }

        // FIXME: The errno is always EINVAL here except with width and height 0!
    }

    pub unsafe fn as_mut_ptr(&self) -> *mut CacaCanvasRaw {
        self.canvas
    }

    pub fn set_color_ansi(&mut self, fg: &AnsiColor, bg: &AnsiColor) {
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

    pub fn blit(&mut self, x: i32, y: i32, source: &Canvas, mask: &Canvas) -> CacaResult {
        let result = unsafe { caca_blit(self.canvas, x, y, source.canvas, mask.canvas) };
        if result == 0 {
            Ok(())
        } else {
            let errno = errno().0;
            match errno {
                libc::EINVAL => Err(CacaError::InvalidMaskSize),
                _            => Err(CacaError::Unknown(errno)),
            }
        }
    }

    pub fn set_boundaries(&mut self, x: i32, y: i32, w: i32, h: i32) -> CacaResult {
        let result = unsafe { caca_set_canvas_boundaries(self.canvas, x, y, w, h) };
        if result == 0 {
            Ok(())
        } else {
            let errno = errno().0;
            match errno {
                libc::EINVAL => Err(CacaError::InvalidSize),
                libc::EBUSY  => Err(CacaError::CanvasInUse),
                libc::ENOMEM => Err(CacaError::NotEnoughMemory),
                _            => Err(CacaError::Unknown(errno)),
            }
        }
    }

    pub fn set_size(&mut self, width: i32, height: i32) -> CacaResult {
        let result = unsafe { caca_set_canvas_size(self.canvas, width, height) };
        // FIXME: The errno is always EINVAL here except with width and height 0!
        if result == 0 {
            Ok(())
        } else {
            let errno = errno().0;
            match errno {
                libc::EINVAL => Err(CacaError::InvalidSize),
                libc::EBUSY  => Err(CacaError::CanvasInUse),
                libc::ENOMEM => Err(CacaError::NotEnoughMemory),
                _            => Err(CacaError::Unknown(errno)),
            }
        }
    }

    pub fn width(&self) -> i32 {
        unsafe { caca_get_canvas_width(self.canvas) as i32 }
    }

    pub fn height(&self) -> i32 {
        unsafe { caca_get_canvas_height(self.canvas) as i32 }
    }

    pub fn frame_count(&self) -> i32 {
        unsafe { caca_get_frame_count(self.canvas) }
    }

    pub fn set_frame(&mut self, frame_index: i32) -> CacaResult {
        let result = unsafe { caca_set_frame(self.canvas, frame_index) };
        if result == 0 {
            Ok(())
        } else {
            let errno = errno().0;
            match errno {
                libc::EINVAL => Err(CacaError::InvalidFrameIndex),
                _            => Err(CacaError::Unknown(errno)),

            }
        }
    }

    pub fn get_frame_name(&self) -> &str {
        unsafe {
            let raw_str = caca_get_frame_name(self.canvas);
            let frame_name = CStr::from_ptr(raw_str);
            frame_name.to_str().unwrap()
        }
    }

    pub fn set_frame_name(&mut self, name: &str) -> CacaResult {
        let name_cstring = CString::new(name).unwrap();
        let result = unsafe { caca_set_frame_name(self.canvas, name_cstring.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            let errno = errno().0;
            match errno {
                libc::ENOMEM => Err(CacaError::NotEnoughMemory),
                _            => Err(CacaError::Unknown(errno)),
            }
        }
    }

    pub fn create_frame(&mut self, frame_index: i32) -> CacaResult {
        let result = unsafe { caca_create_frame(self.canvas, frame_index) };
        if result == 0 {
            Ok(())
        } else {
            let errno = errno().0;
            match errno {
                libc::ENOMEM => Err(CacaError::NotEnoughMemory),
                _            => Err(CacaError::Unknown(errno)),
            }
        }
    }

    pub fn remove_frame(&mut self, frame_index: i32) -> CacaResult {
        let result = unsafe { caca_free_frame(self.canvas, frame_index) };
        if result == 0 {
            Ok(())
        } else {
            let errno = errno().0;
            match errno {
                libc::EINVAL => Err(CacaError::InvalidFrameIndex),
                _            => Err(CacaError::Unknown(errno)),
            }
        }
    }
}

impl<'a> Drop for Canvas<'a> {
    fn drop(&mut self) {
        unsafe {
            caca_free_canvas(self.canvas);
        }
    }
}

#[cfg(test)]
mod tests {
    //! NOTE: These tests have to be run in a single-threaded environment
    //! (RUST_TEST_THREADS=1), because the raw libcaca primitives are not
    //! thread-safe.
    use super::*;

    fn get_canvas_and_display() -> (Canvas<'static>, Display) {
        let canvas = Canvas::new(100, 100);
        assert!(canvas.is_ok(), "{:?}", canvas.err());
        let canvas_ok = canvas.unwrap();
        let display = Display::new(InitOptions{canvas: Some(&canvas_ok),
                                               ..InitOptions::default()});
        assert!(display.is_ok(), "{:?}", display.err());
        (canvas_ok, display.unwrap())
    }

    #[test]
    fn test_resize_canvas() {
        let canvas = Canvas::new(100, 100);

        assert!(canvas.is_ok(),"{:?}", canvas.err());
        let mut canvas = canvas.unwrap();

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
