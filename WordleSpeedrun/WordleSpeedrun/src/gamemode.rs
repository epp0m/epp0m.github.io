use ggez::Context;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Drawable, PxScale, Text, TextFragment};
use crate::{BR, DBR, GREEN, ORANGE, PALE};
use crate::keyboard::VirtualKeyboard;

type GameResult = (bool, usize, f32);

#[derive(Clone, PartialEq)]
pub enum GameMode {
    Classic,
    Streak,
    Assisted,
    Impossible,
}

impl GameMode {
    pub fn display(&self) -> String {
        match self {
            GameMode::Classic => "Classic [>]".to_string(),
            GameMode::Streak => "[<] Streak [>]".to_string(),
            GameMode::Assisted => "[<] Assisted [>]".to_string(),
            GameMode::Impossible => "[<] Impossible".to_string(),
        }
    }

    fn assist(vk: &VirtualKeyboard) -> String {
        "Coming Soon".to_string()
    }
    
    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, in_menu: bool,
                last_games: &[GameResult], vk: &VirtualKeyboard) {
        match self {
            GameMode::Classic => {

            },
            GameMode::Streak => {
                if in_menu {
                    let (w, h) = ctx.gfx.drawable_size();

                    let mut streak = 0usize;
                    let mut average_length = 0usize;
                    let mut average_time = 0f32;
                    for res in last_games.iter().rev() {
                        if res.0 {
                            streak += 1;
                            average_length += res.1;
                            average_time += res.2; } 
                        else { break } }

                    let stats =
                        if streak >= 4 {
                            average_time /= streak as f32;
                            average_length /= streak;
                            format!("{} wins in a row\n{} average time\n{} average length",
                                    streak, average_time, average_length) }
                        else { format!("Win {} more games\nto see stats\nof your streak", 4 - streak) };
                    for (i, line) in stats.lines().enumerate() {
                        let frag = TextFragment::new(line)
                            .color(ORANGE.clone())
                            .scale(PxScale::from(w / DBR)).font("JM");
                        let text = Text::new(frag);
                        let td = text.dimensions(ctx);
                        let pos = Vec2::new((w / 2.5 - td.w) / 2.0, h * 0.8 + td.h * i as f32);
                        canvas.draw(&text, pos);
                    } }
            },
            GameMode::Assisted => {
                let (w, h) = ctx.gfx.drawable_size();
                if in_menu {
                    let stats = "Assisted mode gives you\n\
                    a good word to guess at each step".to_owned();
                    for (i, line) in stats.lines().enumerate() {
                        let frag = TextFragment::new(line)
                            .color(GREEN.clone())
                            .scale(PxScale::from(w / DBR)).font("JM");
                        let text = Text::new(frag);
                        let td = text.dimensions(ctx);
                        let pos = Vec2::new((w / 2.5 - td.w) / 2.0, h * 0.8 + td.h * i as f32);
                        canvas.draw(&text, pos);
                    } }
                else {
                    let word = Self::assist(vk);
                    let frag = TextFragment::new(word)
                        .color(PALE.clone())
                        .scale(PxScale::from(w / BR)).font("JM");
                    let text = Text::new(frag);
                    let td = text.dimensions(ctx);
                    let pos = Vec2::new((w / 2.5 - td.w) / 2.0, h * 0.8);
                    canvas.draw(&text, pos);
                }
            },
            GameMode::Impossible => {
                if in_menu {
                    let (w, h) = ctx.gfx.drawable_size();

                    let stats = "Impossible mode changes the\n\
                    solution when it's found once".to_owned();
                    for (i, line) in stats.lines().enumerate() {
                        let frag = TextFragment::new(line)
                            .color(ORANGE.clone())
                            .scale(PxScale::from(w / DBR)).font("JM");
                        let text = Text::new(frag);
                        let td = text.dimensions(ctx);
                        let pos = Vec2::new((w / 2.5 - td.w) / 2.0, h * 0.8 + td.h * i as f32);
                        canvas.draw(&text, pos);
                    } }
            },
        }
    }
}
