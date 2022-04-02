use bracket_lib::prelude::*;

use super::constants;
use super::obstacle;
use super::player;

pub enum GameMode {
    Menu,
    Playing,
    End,
}

pub struct State {
    pub player: player::Player,
    pub frame_time: f32,
    pub mode: GameMode,
    pub obstacle: obstacle::Obstacle,
    pub score: i32,
}

impl State {
    pub fn new() -> Self {
        State {
            player: player::Player::new(2, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: obstacle::Obstacle::new(constants::SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn restart(&mut self) {
        self.player = player::Player::new(5, constants::SCREEN_WIDTH / 2);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = obstacle::Obstacle::new(constants::SCREEN_WIDTH, 0);
        self.score = 0;
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > constants::FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle =
                obstacle::Obstacle::new(self.player.x + constants::SCREEN_WIDTH, self.score);
        }

        if self.player.y as i32 > constants::SCREEN_HEIGHT
            || self.player.y == 0.0
            || self.obstacle.hit_obstacle(&self.player)
        {
            self.mode = GameMode::End;
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm, msg: &str) {
        ctx.cls();
        ctx.print_color_centered(5, YELLOW, BLACK, msg);
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Game");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx, "Welcome to Flappy Dragon"),
            GameMode::End => self.main_menu(ctx, "You are dead"),
            GameMode::Playing => self.play(ctx),
        }
    }
}
