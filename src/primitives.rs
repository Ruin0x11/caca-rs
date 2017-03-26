use ::CacaCanvas;
use caca::*;

impl<'a> CacaCanvas<'a> {
    pub fn draw_line(&mut self, x: i32, y: i32, w: i32, h: i32, c: char) {
        unsafe { caca_draw_line(self.canvas, x, y, w, h, c as u32) };
    }

    #[cfg(never)]
    pub fn draw_polyline(&mut self, coords: &[(i32, i32)], c: char) {
        unsafe { caca_draw_line(self.canvas, x, y, w, h, c as u32) };
    }

    pub fn draw_thin_line(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { caca_draw_thin_line(self.canvas, x, y, w, h) };
    }

    #[cfg(never)]
    pub fn draw_thin_polyline(&mut self, coords: &[(i32, i32)]) {
        unsafe { caca_draw_thin_polyline(self.canvas, x, y, w, h) };
    }
}
