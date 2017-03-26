#[macro_use] extern crate bitflags;
extern crate errno;
extern crate libc;
extern crate caca_sys as caca;

mod dither;
mod primitives;

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
        match *self {
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

pub enum CharWidth {
    HalfWidth,
    FullWidth,
}

impl DriverType {
    fn from_cstr(cs: &CStr) -> DriverType {
        let string = cs.to_str().unwrap();
        match string {
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

pub struct CacaColor {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

pub type CacaResult = Result<(), CacaError>;

impl CacaDisplay {
    pub fn new(opts: InitOptions) -> Result<Self, CacaError> {
        let canvas_ptr = match opts.canvas {
            Some(canvas_) => unsafe { canvas_.mut_ptr() },
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

        match errno().0 {
            libc::ENOMEM => Err(CacaError::NotEnoughMemory),
            //libc::ENODEV => Err(CacaError::FailedToOpenGraphicsDevice),
            _      => Ok(CacaDisplay {
                display: display,
            }),
        }
    }

    pub fn display_driver(&self) -> DriverType {
        unsafe {
            let raw_str = caca_get_display_driver(self.display);
            let driver_name = CStr::from_ptr(raw_str);
            DriverType::from_cstr(driver_name)
        }
    }

    pub fn set_display_driver(&mut self, driver: DriverType) -> CacaResult {
        let driver_str = driver.to_cstring();
        let result = unsafe { caca_set_display_driver(self.display, driver_str.as_ptr()) };
        if result == -1 {
            Err(CacaError::Unknown)
        } else {
            Ok(())
        }
    }

    #[cfg(never)]
    pub fn driver_list() -> Vec<DriverType> {
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

    // TEMP
    pub fn wait(&self) {
        unsafe { caca_get_event(self.display, caca::CACA_EVENT_KEY_PRESS as i32, null_mut(), -1)};
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

    pub unsafe fn mut_ptr(&self) -> *mut CacaCanvasRaw {
        self.canvas
    }

    pub fn set_color_ansi(&mut self, fg: AnsiColor, bg: AnsiColor) {
        unsafe { caca_set_color_ansi(self.canvas, fg.as_byte(), bg.as_byte()) };
    }

    pub fn put_char(&mut self, x: i32, y: i32, c: char) -> CharWidth {
        let size = unsafe { caca_put_char(self.canvas, x, y, c as u32) };
        if size == 2 {
            CharWidth::FullWidth
        } else {
            CharWidth::HalfWidth
        }
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

    pub fn width(&self) -> usize {
        unsafe { caca_get_canvas_width(self.canvas) as usize }
    }

    pub fn height(&self) -> usize {
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
                                                   driver: Some(DriverType::X11),
                                                   ..InitOptions::default()});
        assert!(display.is_ok(), "{:?}, display.err()");
        (canvas_ok, display.unwrap())
    }

    #[test]
    fn test_resize_canvas() {
        let (mut canvas, _) = get_canvas_and_display();

        canvas.set_size(0, 0);

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

    #[test]
    fn test_display() {
        let (mut canvas, mut display) = get_canvas_and_display();
        canvas.set_color_ansi(AnsiColor::White, AnsiColor::Blue);
        canvas.put_char(10, 10, 'x');
        canvas.put_str(10, 11, "Doods! String!");
        display.refresh();
        display.wait();
    }
}
