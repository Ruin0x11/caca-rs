use ::Canvas;
use caca::*;

impl<'a> Canvas<'a> {
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

    pub fn draw_circle(&mut self, x: i32, y: i32, r: i32, c: char) {
        unsafe { caca_draw_circle(self.canvas, x, y, r, c as u32) };
    }

    pub fn draw_ellipse(&mut self, x: i32, y: i32, a: i32, b: i32, c: char) {
        unsafe { caca_draw_ellipse(self.canvas, x, y, a, b, c as u32) };
    }

    pub fn draw_thin_ellipse(&mut self, x: i32, y: i32, a: i32, b: i32) {
        unsafe { caca_draw_thin_ellipse(self.canvas, x, y, a, b) };
    }

    pub fn fill_ellipse(&mut self, x: i32, y: i32, a: i32, b: i32, c: char) {
        unsafe { caca_fill_ellipse(self.canvas, x, y, a, b, c as u32) };
    }

    pub fn draw_box(&mut self, x: i32, y: i32, w: i32, h: i32, c: char) {
        unsafe { caca_draw_box(self.canvas, x, y, w, h, c as u32) };
    }

    pub fn draw_thin_box(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { caca_draw_thin_box(self.canvas, x, y, w, h) };
    }

    pub fn draw_cp437_box(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { caca_draw_cp437_box(self.canvas, x, y, w, h) };
    }

    pub fn fill_box(&mut self, x: i32, y: i32, w: i32, h: i32, c: u32) {
        unsafe { caca_fill_box(self.canvas, x, y, w, h, c) };
    }

    pub fn draw_triangle(&mut self, coords: &[(i32, i32); 3], c: char) {
        unsafe { caca_draw_triangle(self.canvas,
                                    coords[0].0, coords[0].1,
                                    coords[1].0, coords[1].1,
                                    coords[2].0, coords[2].1,
                                    c as u32) };
    }

    pub fn draw_thin_triangle(&mut self, coords: &[(i32, i32); 3]) {
        unsafe { caca_draw_thin_triangle(self.canvas,
                                         coords[0].0, coords[0].1,
                                         coords[1].0, coords[1].1,
                                         coords[2].0, coords[2].1) };
    }

    pub fn fill_triangle(&mut self, coords: &[(i32, i32); 3], c: char) {
        unsafe { caca_fill_triangle(self.canvas,
                                    coords[0].0, coords[0].1,
                                    coords[1].0, coords[1].1,
                                    coords[2].0, coords[2].1,
                                    c as u32) };
    }

    #[cfg(never)]
    pub fn fill_triangle_textured(&mut self, coords: &[(i32, i32); 3],
                                  tex: &Canvas,
                                  tex_coords: &[(i32, i32); 3]) {
        unsafe { caca_fill_triangle(self.canvas,
                                    coords[0].0, coords[0].1,
                                    coords[1].0, coords[1].1,
                                    coords[2].0, coords[2].1,
                                    c as u32) };
    }
}
