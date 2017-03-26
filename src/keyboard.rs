use std::char;

use caca::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Backspace,
    Tab,
    Return,
    Pause,
    Escape,
    Delete,
    Up,
    Down,
    Left,
    Right,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,

    Char(char),
    Ctrl(char),
    Function(u32),
    Unknown(i32),
}

impl Key {
    pub fn from_code(code: i32) -> Key {
        match code {
            CACA_KEY_CTRL_A    => Key::Ctrl('a'),
            CACA_KEY_CTRL_B    => Key::Ctrl('b'),
            CACA_KEY_CTRL_C    => Key::Ctrl('c'),
            CACA_KEY_CTRL_D    => Key::Ctrl('d'),
            CACA_KEY_CTRL_E    => Key::Ctrl('e'),
            CACA_KEY_CTRL_F    => Key::Ctrl('f'),
            CACA_KEY_CTRL_G    => Key::Ctrl('g'),
            CACA_KEY_BACKSPACE => Key::Backspace,
            CACA_KEY_TAB       => Key::Tab,
            CACA_KEY_CTRL_J    => Key::Ctrl('j'),
            CACA_KEY_CTRL_K    => Key::Ctrl('k'),
            CACA_KEY_CTRL_L    => Key::Ctrl('l'),
            CACA_KEY_RETURN    => Key::Return,
            CACA_KEY_CTRL_N    => Key::Ctrl('n'),
            CACA_KEY_CTRL_O    => Key::Ctrl('o'),
            CACA_KEY_CTRL_P    => Key::Ctrl('p'),
            CACA_KEY_CTRL_Q    => Key::Ctrl('q'),
            CACA_KEY_CTRL_R    => Key::Ctrl('r'),
            CACA_KEY_PAUSE     => Key::Pause,
            CACA_KEY_CTRL_T    => Key::Ctrl('t'),
            CACA_KEY_CTRL_U    => Key::Ctrl('u'),
            CACA_KEY_CTRL_V    => Key::Ctrl('v'),
            CACA_KEY_CTRL_W    => Key::Ctrl('w'),
            CACA_KEY_CTRL_X    => Key::Ctrl('x'),
            CACA_KEY_CTRL_Y    => Key::Ctrl('y'),
            CACA_KEY_CTRL_Z    => Key::Ctrl('z'),
            CACA_KEY_ESCAPE    => Key::Escape,
            CACA_KEY_DELETE    => Key::Delete,
            CACA_KEY_UP        => Key::Up,
            CACA_KEY_DOWN      => Key::Down,
            CACA_KEY_LEFT      => Key::Left,
            CACA_KEY_RIGHT     => Key::Right,
            CACA_KEY_INSERT    => Key::Insert,
            CACA_KEY_HOME      => Key::Home,
            CACA_KEY_END       => Key::End,
            CACA_KEY_PAGEUP    => Key::PageUp,
            CACA_KEY_PAGEDOWN  => Key::PageDown,
            CACA_KEY_F1        => Key::Function(1),
            CACA_KEY_F2        => Key::Function(2),
            CACA_KEY_F3        => Key::Function(3),
            CACA_KEY_F4        => Key::Function(4),
            CACA_KEY_F5        => Key::Function(5),
            CACA_KEY_F6        => Key::Function(6),
            CACA_KEY_F7        => Key::Function(7),
            CACA_KEY_F8        => Key::Function(8),
            CACA_KEY_F9        => Key::Function(9),
            CACA_KEY_F10       => Key::Function(10),
            CACA_KEY_F11       => Key::Function(11),
            CACA_KEY_F12       => Key::Function(12),
            CACA_KEY_F13       => Key::Function(13),
            CACA_KEY_F14       => Key::Function(14),
            CACA_KEY_F15       => Key::Function(15),
            CACA_KEY_UNKNOWN   => Key::Unknown(code),
            _                  => {
                match char::from_u32(code as u32) {
                    Some(chr) => Key::Char(chr),
                    None    => Key::Unknown(code),
                }
            }
        }
    }
}
