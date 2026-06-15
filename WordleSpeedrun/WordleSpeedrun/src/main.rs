mod menu;
mod keyboard;
mod gamestate;
mod gamemode;

use gamestate::*;
use ggez::{event, Context, ContextBuilder, GameResult};
use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Drawable, PxScale, Text, TextFragment};
use ggez::input::keyboard::{Key, KeyInput};
use ggez::winit::keyboard::NamedKey;
use lazy_static::lazy_static;
use crate::gamemode::GameMode;

const TR: f32 = 18.0;
const BR: f32 = 20.0;
const DBR: f32 = BR * 2.0;


lazy_static! {
    static ref DARK_IMPOSSIBLE: Color = Color::from_rgb(80, 20, 4);
    static ref BG_IMPOSSIBLE: Color = Color::from_rgb(50, 22, 2);
    static ref TILE_IMPOSSIBLE: Color = Color::from_rgb(65, 44, 21);
    static ref TILE: Color = Color::from_rgb(44, 44, 58);
    static ref DARK: Color = Color::from_rgb(5, 7, 12);
    static ref BG: Color = Color::from_rgb(22, 22, 29);
    static ref DARK_GREEN: Color = Color::from_rgb(0, 150, 20);
    static ref GREEN: Color = Color::from_rgb(80, 245, 120);
    static ref LIME: Color = Color::from_rgb(150, 255, 160);
    static ref ORANGE: Color = Color::from_rgb(255, 150, 20);
    static ref RED: Color = Color::from_rgb(220, 10, 20);
    static ref PALE: Color = Color::from_rgb(230, 224, 218);
    
    static ref VALID_GUESSES: Vec<&'static str> = {
        include_str!("valid_guesses.txt").lines().map(|s| s.trim()).collect() };
    static ref SOLUTIONS: Vec<&'static str> = {
        include_str!("solutions.txt").lines().map(|s| s.trim()).collect() };
}


impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(Self::FPS) {
            if self.menu.in_menu {

            }
            else {
                if self.new_input {
                    let lines = self.letters.len() / 5;
                    if lines > 0 {
                        let last_guess = &self.letters.chars().collect::<Vec<_>>()[(lines - 1) * 5..lines * 5];
                        let sol_chars = self.solution.chars().collect::<Vec<_>>();
                        if last_guess == sol_chars {
                            self.menu.in_menu = true;
                            self.menu.previous_game_results.push((true, lines, self.time));
                            self.menu.win = Some(true);
                            self.menu.time = Some(self.time); } } }
                self.time += ctx.time.delta().as_secs_f32();
                self.menu.time = Some(self.time);
                if self.letters.len() == 30 && self.new_input && !self.menu.in_menu {
                    self.menu.in_menu = true;
                    self.menu.previous_game_results.push((false, 6, self.time));
                    self.menu.win = Some(false); } } }

        Ok(())
    }

    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(
            ctx, if self.menu.is_impossible() { BG_IMPOSSIBLE.clone() } else { BG.clone() });
        self.vk.draw(&mut canvas, ctx, self.menu.is_impossible());

        let (w, h) = ctx.gfx.drawable_size();
        let start_x = w / 2.0 - w/TR * 2.5;
        let start_y = h / 2.0 - w/TR * 3.0;
        for mesh in &self.tile_meshes { canvas.draw(mesh, Vec2::ZERO); }

        for (i, ch) in self.letters.chars().enumerate() {
            let x = start_x + ((i % 5) as f32 * w / TR);
            let y = start_y + ((i / 5) as f32 * w / TR);

            let ch = ch.to_string();
            let lc = &ch.to_lowercase();
            let text_fragment = TextFragment::new(ch.to_uppercase())
                .color(if !self.new_input && (i / 5 == (self.letters.len() - 1) / 5) &&
                    ((self.vk.wrong.contains(lc) && !self.vk.correct.contains(lc) && !self.vk.found.contains(lc)) ||
                    (self.vk.correct.contains(lc) && !self.vk.found.contains(lc) && self.letters.chars().enumerate()
                        .any(|(j, c)| { j % 5 == i % 5 && c.to_string() == ch && j != i }))) { RED.clone() }
                       else { Color::WHITE }).scale(PxScale::from(w / TR * 0.75)).font("JM");
            let text = Text::new(text_fragment);

            let text_dimensions = text.dimensions(ctx);
            let text_x = x + (w / TR - text_dimensions.w) / 2.0;
            let text_y = y + (w / TR - text_dimensions.h) / 2.0;
            canvas.draw(&text, Vec2::new(text_x, text_y));

            if i % 5 == 4 {
                let row = i / 5;
                let time = if row == 0 { self.times[0] }
                else { self.times[row] - self.times[row - 1] };
                if time > 0.0 {
                    let time_fragment = TextFragment::new(format!("{:.1}", time))
                        .color(PALE.clone()).scale(PxScale::from(w / TR * 0.5)).font("JM");
                    let time_text = Text::new(time_fragment);
                    canvas.draw(&time_text, Vec2::new(start_x + w / TR * 5.0, text_y)); } } }

        self.menu.draw(&mut canvas, ctx, self.solution.clone(), &self.vk);
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _: bool) -> GameResult {
        let char = input.event.logical_key.to_text();
        
        if input.event.logical_key == Key::Named(NamedKey::Space) && self.menu.in_menu {
            let layout = self.vk.layout.clone();
            let menu = self.menu.clone();
            *self = GameState::new(ctx);
            self.vk.layout = layout;
            self.menu = menu;
            self.menu.in_menu = false;
            if !self.menu.is_streak() { self.menu.previous_game_results.clear(); }
            self.update_colors(ctx)
        }
        
        else if input.event.logical_key == Key::Named(NamedKey::Space) && !self.menu.in_menu {
            self.menu.in_menu = true; }
        else if input.event.logical_key == Key::Named(NamedKey::Tab) { self.vk.swap_layout() }
        else if input.event.logical_key == Key::Named(NamedKey::Escape) { ctx.request_quit(); }
        else if input.event.logical_key == Key::Named(NamedKey::Enter) && !self.menu.in_menu {
            let y = self.letters.len() / 5;
            if y > 0 { let guess = self.letters.clone().split_off((y - 1) * 5);
                if VALID_GUESSES.iter().any(|&x| x == &guess) {
                    self.guessed(ctx); } } }
        
        else if input.event.logical_key == Key::Named(NamedKey::Backspace) {
            if !(self.letters.len() % 5 == 0 && self.new_input) { self.letters.pop(); }
            if self.letters.len() % 5 == 0 { self.new_input = true; } }

        else if input.event.logical_key == Key::Named(NamedKey::ArrowRight) && self.menu.in_menu {
            match self.menu.gamemode {
                GameMode::Classic => self.menu.gamemode = GameMode::Streak,
                GameMode::Streak => self.menu.gamemode = GameMode::Assisted,
                GameMode::Assisted => self.menu.gamemode = GameMode::Impossible,
                GameMode::Impossible => () } 
            self.update_colors(ctx)
        }

        else if input.event.logical_key == Key::Named(NamedKey::ArrowLeft) && self.menu.in_menu {
            match self.menu.gamemode {
                GameMode::Classic => (),
                GameMode::Streak => self.menu.gamemode = GameMode::Classic,
                GameMode::Assisted => self.menu.gamemode = GameMode::Streak,
                GameMode::Impossible => self.menu.gamemode = GameMode::Assisted }
            self.update_colors(ctx)
        }

        else if char.is_some() && !self.menu.in_menu {
            if self.new_input || self.letters.len() % 5 != 0 {
                self.letters.push(char.unwrap().chars().next().unwrap());
                self.new_input = false; } }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let (mut ctx, events_loop) = ContextBuilder::new("", "")
        .window_setup(WindowSetup::default().title(""))
        .window_mode(WindowMode::default().fullscreen_type(FullscreenType::Desktop)).build()?;

    let state = GameState::new(&mut ctx);
    event::run(ctx, events_loop, state)
}
