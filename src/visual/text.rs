extern crate opengl_graphics;
extern crate piston_window;

use graphics::character::CharacterCache;
use graphics::types::FontSize;
use graphics::{Context, Text};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextAlignment {
    Left,
    Right,
    Center,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextVerticalAlignment {
    Top,
    Bottom,
    Center,
}

pub trait DrawText {
    fn draw_text(
        &mut self,
        text: &str,
        r: [f64; 4],
        color: [f32; 4],
        size: FontSize,
        halign: TextAlignment,
        valign: TextVerticalAlignment,
        glyphs: &mut GlyphCache,
        c: &Context,
    );
}

impl DrawText for GlGraphics {
    fn draw_text(
        &mut self,
        text: &str,
        r: [f64; 4],
        color: [f32; 4],
        size: FontSize,
        halign: TextAlignment,
        valign: TextVerticalAlignment,
        glyphs: &mut GlyphCache,
        c: &Context,
    ) {
        let x0 = r[0];
        let y0 = r[1];
        let x1 = r[2];
        let y1 = r[3];

        let t = Text::new_color(color, size);
        let size = t.measure(text, glyphs).unwrap();
        fn center_w(p0: f64, p1: f64, w: f64) -> f64 {
            p0 + ((p1 - p0) / 2.0) - (w / 2.0)
        }
        fn center_h(p0: f64, p1: f64, h: f64) -> f64 {
            p0 + ((p1 - p0) / 2.0) + (h / 2.0)
        }

        let x = match halign {
            TextAlignment::Left => x0,
            TextAlignment::Right => x1 - size.width,
            TextAlignment::Center => center_w(x0, x1, size.width),
        };

        let y = match valign {
            TextVerticalAlignment::Top => y0,
            TextVerticalAlignment::Bottom => y1 - size.height,
            TextVerticalAlignment::Center => center_h(y0, y1, size.height),
        };

        let transform = c.transform.trans(x, y);
        let draw_state = c.draw_state;
        t.draw(text, glyphs, &draw_state, transform, self).unwrap();
    }
}

pub trait MeasureText {
    fn measure<C>(&self, text: &str, cache: &mut C) -> Result<Size, ()>
    where
        C: CharacterCache;
}

impl MeasureText for Text {
    fn measure<C>(&self, text: &str, cache: &mut C) -> Result<Size, ()>
    where
        C: CharacterCache,
    {
        let mut w = 0.0;
        let mut h = 0.0;
        for ch in text.chars() {
            let character = cache.character(self.font_size, ch).ok().unwrap();
            let (left, top) = (character.left(), character.top());
            w += character.advance_width() + left;
            h = (character.advance_height() + top).max(h);
        }
        let result = Size {
            width: w as f64,
            height: h as f64,
        };
        Ok(result)
    }
}
