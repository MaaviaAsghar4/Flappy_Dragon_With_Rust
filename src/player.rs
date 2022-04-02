use bracket_lib::prelude::*;

use super::constants;

pub struct Player {
    pub x: i32,
    pub y: f32,
    pub velocity: f32,
    pub frame: usize,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y: y as f32,
            velocity: 0.0,
            frame: 0,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF::new(0.0, self.y),
            1,
            Degrees::new(0.0),
            PointF::new(2.0, 2.0),
            WHITE,
            NAVY,
            constants::DRAGON_FRAMES[self.frame],
        );
        ctx.set_active_console(0);
    }

    pub fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.1;
        }
        self.y += self.velocity;
        if self.y < 0.0 {
            self.y = 0.0;
        }
        self.x += 1;
        self.frame += 1;
        self.frame = self.frame % 6;
    }

    pub fn flap(&mut self) {
        self.velocity = -1.0;
    }
}
