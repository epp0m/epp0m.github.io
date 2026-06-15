use ggez::Context;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, DrawMode, Drawable, Mesh, PxScale, Rect, Text, TextFragment};
use crate::{BR, DARK, DARK_IMPOSSIBLE, DBR, GREEN, PALE, RED, TR};
use crate::gamemode::GameMode;
use crate::keyboard::VirtualKeyboard;

type GameResult = (bool, usize, f32);

#[derive(Clone)]
pub struct GameMenu {
    pub in_menu: bool,
    pub time: Option<f32>,
    pub win: Option<bool>,
    pub gamemode: GameMode,
    pub previous_game_results: Vec<GameResult>,
}

impl GameMenu {
    pub fn new() -> Self {
        Self { time: None, win: None, gamemode: GameMode::Classic,
            in_menu: true, previous_game_results: Vec::new() }
    }

    pub fn is_impossible(&self) -> bool {
        self.gamemode == GameMode::Impossible
    }

    pub fn is_streak(&self) -> bool {
        self.gamemode == GameMode::Streak
    }
    
    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, solution: String, vk: &VirtualKeyboard) {
        self.gamemode.draw(ctx, canvas, self.in_menu, &self.previous_game_results, vk);
        
        if !self.in_menu { return; }
        
        let (w, h) = ctx.gfx.drawable_size();
        let rect = Rect::new(w / 4.0, h / TR, w / 2.0, h / TR * 2.0);
        let floating = Mesh::new_rounded_rectangle(
            ctx, DrawMode::fill(), rect, w / DBR,
            if self.is_impossible() { DARK_IMPOSSIBLE.clone() }
            else { DARK.clone() }).unwrap();
        canvas.draw(&floating, Vec2::ZERO);
        let status_text = if let Some(win) = self.win {
            if win { &format!("You Won in {:.2} seconds", self.time.unwrap()) }
            else { &format!("You missed ''{solution}''... :(") } }
        else { "Welcome! (by lygzounette)" };
        let frag = if let Some(win) = self.win {
            TextFragment::new(status_text).color(if win { GREEN.clone() }
            else { RED.clone() }).scale(PxScale::from(w / BR)).font("JM") }
        else { TextFragment::new(status_text)
            .color(PALE.clone()).scale(PxScale::from(w / BR)).font("JM") };

        let text = Text::new(frag);
        let td = text.dimensions(ctx);
        let pos = Vec2::new((w - td.w )/ 2.0, h / (TR * 0.85));
        canvas.draw(&text, pos);

        let message = if self.time.is_some() { "Space to reset\nEscape to leave" }
        else { "Space to start\nTab to change keyboard" };
        for (i, line) in message.lines().enumerate() {
            let frag = TextFragment::new(line)
                .color(PALE.clone())
                .scale(PxScale::from(w / DBR)).font("JM");
            let text = Text::new(frag);
            let td = text.dimensions(ctx);
            let pos = Vec2::new((w - td.w) / 2.0, h * 0.8 + td.h * i as f32);
            canvas.draw(&text, pos);
        }
        let gamemode = self.gamemode.display();
        let frag = TextFragment::new(gamemode)
            .color(GREEN.clone())
            .scale(PxScale::from(w / DBR)).font("JM");
        let text = Text::new(frag);
        let td = text.dimensions(ctx);
        let pos = Vec2::new((w - td.w) / 2.0, h * 0.9);
        canvas.draw(&text, pos);
    }
}
