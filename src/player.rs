use bracket_lib::prelude::*;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub velocity: f32,
}


impl Player {
    pub fn new(x:i32, y:i32) -> Self {
        Player{x: x, y: y, velocity: 0.0}
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    pub fn update(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    pub fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

