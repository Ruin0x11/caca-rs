use std::mem;
use std::time::Duration;
use libc::{c_int, c_char};

use ::{CacaDisplay, CacaEventRaw};
use caca::*;
use keyboard::Key;

#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    None,
    Left,
    Right,
    Middle,
    Other(i32),
}

#[derive(Clone, Copy, Debug)]
pub struct Mouse {
    button: MouseButton,
    x: i32,
    y: i32,
}

impl Into<Key> for KeyEventRaw {
    fn into(self) -> Key {
        Key::from_code(self.ch)
    }
}

impl Into<Mouse> for MouseEventRaw {
    fn into(self) -> Mouse {
        let button = match self.button {
            1 => MouseButton::Left,
            2 => MouseButton::Right,
            3 => MouseButton::Middle,
            _ => MouseButton::Other(self.button),
        };

        Mouse {
            button: button,
            x: self.x,
            y: self.y
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    KeyPress(Key),
    KeyRelease(Key),
    MousePress(Mouse),
    MouseRelease(Mouse),
    MouseMotion(Mouse),
    Resize(i32, i32),
    Quit,
    Any,
    Unknown(u32),
}

bitflags! {
    pub flags EventMask: u32 {
        const EVENT_NONE          = CACA_EVENT_NONE,
        const EVENT_KEY_PRESS     = CACA_EVENT_KEY_PRESS,
        const EVENT_KEY_RELEASE   = CACA_EVENT_KEY_RELEASE,
        const EVENT_MOUSE_PRESS   = CACA_EVENT_MOUSE_PRESS,
        const EVENT_MOUSE_RELEASE = CACA_EVENT_MOUSE_RELEASE,
        const EVENT_MOUSE_MOTION  = CACA_EVENT_MOUSE_MOTION,
        const EVENT_RESIZE        = CACA_EVENT_RESIZE,
        const EVENT_QUIT          = CACA_EVENT_QUIT,
        const EVENT_ANY           = CACA_EVENT_ANY,
   }
}

const NIL_RAW_EVENT: CacaEventRaw = CacaEventRaw { type_: 0, data: [0, 0, 0, 0,
                                                                    0, 0, 0, 0,
                                                                    0, 0, 0, 0,
                                                                    0, 0, 0, 0] };

impl CacaDisplay {
    pub fn poll_event(&self, mask: u32) -> Option<Event> {
        let mut ev = NIL_RAW_EVENT;
        unsafe { caca_get_event(self.display, mask as c_int, &mut ev, -1)};
        unpack_event(&ev)
    }

    pub fn peek_event(&self, mask: u32, usecs: Duration) -> Option<Event> {
        let mut ev = NIL_RAW_EVENT;
        let timeout = (usecs.as_secs() * 1000 + usecs.subsec_nanos() as u64 / 1000000) as c_int;
        unsafe { caca_get_event(self.display, mask as c_int, &mut ev, timeout) };
        unpack_event(&ev)
    }
}

pub fn unpack_event(event: &CacaEventRaw) -> Option<Event> {
    match event.type_ {
        CACA_EVENT_NONE          => None,
        CACA_EVENT_KEY_PRESS     => Some(Event::KeyPress(key_from_raw(event))),
        CACA_EVENT_KEY_RELEASE   => Some(Event::KeyRelease(key_from_raw(event))),
        CACA_EVENT_MOUSE_PRESS   => Some(Event::MousePress(mouse_from_raw(event))),
        CACA_EVENT_MOUSE_RELEASE => Some(Event::MouseRelease(mouse_from_raw(event))),
        CACA_EVENT_MOUSE_MOTION  => Some(Event::MouseMotion(mouse_from_raw(event))),
        CACA_EVENT_RESIZE        => {
            let (w, h) = resize_from_raw(event);
            Some(Event::Resize(w, h))
        },
        CACA_EVENT_QUIT          => Some(Event::Quit),
        CACA_EVENT_ANY           => Some(Event::Any),
        _                        => Some(Event::Unknown(event.type_)),
    }
}

// Because there is no support for unions in Rust, the union types have to be
// manually transmuted.
#[repr(C)]
struct MouseEventRaw {
    x: c_int,
    y: c_int,
    button: c_int,
}

#[repr(C)]
struct ResizeEventRaw {
    w: c_int,
    h: c_int,
}

#[repr(C)]
struct KeyEventRaw {
    ch: c_int,
    utf32: u32,
    utf8: [c_char; 8],
}

fn key_from_raw(ev: &CacaEventRaw) -> Key {
    let raw = unsafe { transmute_key_event(ev.data) };
    raw.into()
}

fn mouse_from_raw(ev: &CacaEventRaw) -> Mouse {
    let raw = unsafe { transmute_mouse_event(ev.data) };
    raw.into()
}

fn resize_from_raw(ev: &CacaEventRaw) -> (i32, i32) {
    let raw = unsafe { transmute_resize_event(ev.data) };
    (raw.w, raw.h)
}

// NOTE: Any way to specify exact size of struct, to allow generic raw event
// type?

unsafe fn transmute_key_event(data: [u8; 16]) -> KeyEventRaw {
    mem::transmute::<[u8; 16], KeyEventRaw>(data)
}

unsafe fn transmute_mouse_event(data: [u8; 16]) -> MouseEventRaw {
    let mut arr: [u8; 12] = Default::default();
    arr.copy_from_slice(&data[0..12]);
    mem::transmute::<[u8; 12], MouseEventRaw>(arr)
}

unsafe fn transmute_resize_event(data: [u8; 16]) -> ResizeEventRaw {
    let mut arr: [u8; 8] = Default::default();
    arr.copy_from_slice(&data[0..8]);
    mem::transmute::<[u8; 8], ResizeEventRaw>(arr)
}
