use ggez::Context;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Drawable, Mesh, PxScale, Rect, Text, TextFragment};
use ggez::mint::Point2;
use crate::{BG_IMPOSSIBLE, DARK, DARK_GREEN, DARK_IMPOSSIBLE, BG, LIME, ORANGE, TR};

pub struct VirtualKeyboard {
    pub found: String,
    pub correct: String,
    pub wrong: String,
    pub layout: KeyboardLayout,
}

#[derive(Clone)]
pub enum KeyboardLayout {
    Qwerty,
    Azerty,
}

impl VirtualKeyboard {
    pub fn new() -> Self {
        Self {
            found: String::new(),
            correct: String::new(),
            wrong: String::new(),
            layout: KeyboardLayout::Qwerty,
        }
    }

    fn get_layout_rows(&self) -> Vec<&str> {
        match self.layout {
            KeyboardLayout::Qwerty => vec!["qwertyuiop", "asdfghjkl", "zxcvbnm"],
            KeyboardLayout::Azerty => vec!["azertyuiop", "qsdfghjklm", "wxcvbn"],
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, is_impossible: bool) {
        let (w, h) = ctx.gfx.drawable_size();
        let keyboard_width = w / 1.5;

        let rows = self.get_layout_rows();
        let size = keyboard_width / 10.0;

        for (row_index, row_letters) in rows.iter().enumerate() {
            let row_y = h / 2.0 + (row_index as f32 - 1.5) * size;

            let mid_point = row_letters.len() + 1 >> 1;
            let left_half = &row_letters[..mid_point];
            let right_half = &row_letters[mid_point..];

            for (col_index, ch) in left_half.chars().rev().enumerate() {
                let x = w / 2.0 - w / TR * 4.0 - col_index as f32 * size;
                self.draw_key(ctx, canvas, x, row_y, size, ch, is_impossible);
            }

            for (col_index, ch) in right_half.chars().enumerate() {
                let x = w / 2.0 + w / TR * 3.0 + col_index as f32 * size;
                self.draw_key(ctx, canvas, x, row_y, size, ch, is_impossible);
            }


        }
    }

    fn draw_key(&self, ctx: &mut Context, canvas: &mut Canvas, x: f32, y: f32, size: f32, ch: char, is_impossible: bool) {
        let key_rect = Rect::new(x, y, size * 0.9, size * 0.9);
        let (bg_color, key_color) =
            if self.found.contains(ch) { (DARK_GREEN.clone(), LIME.clone()) }
            else if self.correct.contains(ch) {
                if is_impossible { (DARK_IMPOSSIBLE.clone(), ORANGE.clone()) }
                else { (DARK.clone(), ORANGE.clone()) }
            }
            else if self.wrong.contains(ch) {
                if is_impossible { (BG_IMPOSSIBLE.clone(), BG_IMPOSSIBLE.clone()) }
                else { (BG.clone(), BG.clone()) } }
            else { 
                if is_impossible { (DARK_IMPOSSIBLE.clone(), Color::WHITE) }
                else { (DARK.clone(), Color::WHITE) }};

        let key_mesh = Mesh::new_rounded_rectangle(
            ctx, DrawMode::fill(), key_rect, size * 0.2, bg_color).unwrap();
        let letter_fragment = TextFragment::new(ch.to_uppercase().to_string())
            .color(key_color).scale(PxScale::from(size * 0.6)).font("JM");
        let letter = Text::new(letter_fragment);
        let td = letter.dimensions(ctx);
        let text_x = x + (size - td.w) / 2.0;
        let text_y = y + (size - td.h) / 2.0;
        canvas.draw(&key_mesh, DrawParam::default());
        canvas.draw(&letter, Point2 { x: text_x, y: text_y });
    }

    pub fn swap_layout(&mut self) {
        self.layout = match self.layout {
            KeyboardLayout::Qwerty => KeyboardLayout::Azerty,
            KeyboardLayout::Azerty => KeyboardLayout::Qwerty,
        }
    }
}
