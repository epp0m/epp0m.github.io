use ggez::Context;
use ggez::graphics::{Color, DrawMode, FontData, Mesh, Rect};
use rand::random_range;
use crate::{DARK, DARK_GREEN, DARK_IMPOSSIBLE, ORANGE, SOLUTIONS, TILE, TILE_IMPOSSIBLE, TR};
use crate::gamemode::GameMode;
use crate::keyboard::VirtualKeyboard;
use crate::menu::GameMenu;

pub struct GameState {
    pub menu: GameMenu,
    pub vk: VirtualKeyboard,
    pub letters: String,
    pub tile_meshes: Vec<Mesh>,
    pub new_input: bool,
    pub solution: String,
    pub times: [f32; 6],
    pub time: f32,
}

impl GameState {
    pub const FPS: u32 = 60;

    fn tile_mesh(ctx: &mut Context, x: f32, y: f32, color: Color) -> Mesh {
        let (w, h) = ctx.gfx.drawable_size();
        let start_x = w / 2.0 - w / TR * 2.5;
        let start_y = h / 2.0 - w / TR * 3.0;
        let tile_rect = Rect::new(
            start_x + x * w / TR, start_y + y * w / TR,
            w / TR * 0.95, w / TR * 0.95);
        Mesh::new_rounded_rectangle(ctx, DrawMode::fill(), tile_rect, w / TR / 5.0, color).unwrap()
    }

    pub fn create_tiles(&mut self, ctx: &mut Context, tile_color: Color) {
        let mut tile_meshes = Vec::new();
        for y in 0..6 { for x in 0..5 {
            tile_meshes.push(Self::tile_mesh(ctx, x as f32, y as f32, tile_color)); } }
        self.tile_meshes = tile_meshes;
    }
    
    pub fn new(ctx: &mut Context) -> Self {
        let font_data = FontData::from_path(ctx, "/JuanMikes.otf".to_string()).unwrap();
        ctx.gfx.add_font("JM", font_data);
        let tile_meshes = Vec::new();

        let time = 0f32;
        let times = [time; 6];
        let solution = String::new();
        let menu = GameMenu::new();
        let virtual_keyboard = VirtualKeyboard::new();
        let mut state = Self {
            letters: String::new(), new_input: true, time, tile_meshes,
            solution, times, menu, vk: virtual_keyboard };
        state.create_tiles(ctx, TILE.clone());
        state.set_random_solution();
        // state.solution = "guess".to_owned();
        state
    }

    pub fn set_random_solution(&mut self) {
        self.solution = SOLUTIONS[random_range(0..SOLUTIONS.len())].to_owned();
    }

    pub fn update_colors(&mut self, ctx: &mut Context) {
        if self.menu.is_impossible() {
            self.create_tiles(ctx, TILE_IMPOSSIBLE.clone());
        }
        else {
            self.create_tiles(ctx, TILE.clone());
        }
    }
    
    pub fn guessed(&mut self, ctx: &mut Context) {
        self.new_input = true;
        let y = self.letters.len() / 5;
        if self.letters.len() % 5 == 0 {
            if y > 0 && self.times[y - 1] == 0.0 { self.times[y - 1] = self.time; }
        }

        let guess = &self.letters.chars().map(
            |x| x.to_lowercase().next().unwrap())
            .collect::<Vec<_>>()[(y - 1) * 5..y * 5];
        let sol_chars = self.solution.chars().collect::<Vec<_>>();

        let mut matched = vec![false; 5];
        for x in 0..5 {
            let char = guess[x];
            if char == sol_chars[x] {
                matched[x] = true;
                self.tile_meshes[(y - 1) * 5 + x] = Self::tile_mesh(
                    ctx, x as f32, y as f32 - 1.0, DARK_GREEN.clone());
                if sol_chars.iter().filter(|x| **x == char).count() >
                    self.vk.found.chars().filter(|x| *x == char).count() {
                    self.vk.found.push(char);
                }
            }
        }

        if matched.iter().all(|x| *x) && self.letters.len() == 5 {
            self.set_random_solution();       
            self.vk.found.clear();
            self.vk.correct.clear();
            self.vk.wrong.clear();

            return self.guessed(ctx);
        }
        
        if matched.iter().all(|x| *x) && self.menu.is_impossible() {
            self.set_random_solution();
            self.vk.found.clear();
            self.vk.correct.clear();
            self.vk.wrong.clear();
            
            self.create_tiles(ctx, TILE.clone());
            self.menu.gamemode = GameMode::Classic;

            let chars = self.letters.to_owned().chars().collect::<Vec<_>>();
            let lines = chars.chunks(5);
            self.letters.clear();
            for line in lines {
                self.letters += &line.iter().collect::<String>();
                self.guessed(ctx);
            }

            return;
        }
        


        for x in 0..5 {
            if guess[x] != sol_chars[x] {
                let mut found = false;
                for (j, &sol_c) in sol_chars.iter().enumerate() {
                    if !matched[j] && sol_c == guess[x] {
                        self.vk.correct.push(guess[x]);
                        matched[j] = true;
                        found = true;
                        break;
                    }
                }
                if !found { self.vk.wrong.push(guess[x]); }
                let color = if found { ORANGE.clone() } else
                { if self.menu.is_impossible() { DARK_IMPOSSIBLE.clone() }
                else { DARK.clone() } };
                self.tile_meshes[(y - 1) * 5 + x] = Self::tile_mesh(
                    ctx, x as f32, y as f32 - 1.0, color); 
            } 
        }
        self.vk.correct = self.vk.correct.chars().filter(|x|
            self.vk.found.chars().filter(|y| *y == *x).count() !=
                self.solution.chars().filter(|y| *y == *x).count())
            .collect::<String>();
    }
}
