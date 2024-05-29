use bracket_lib::color::{BLACK, YELLOW};
use bracket_lib::prelude::{to_cp437, BTerm};

pub struct Player {
    pub(crate) x: i32,
    pub(crate) y: i32,
    velocity: f32,
    pub(crate) x_speed: i32,
    pub(crate) start_score: i32,
}

impl Player {
    pub(crate) fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y,
            velocity: 0.0,
            x_speed: 0,
            start_score: 0,
        }
    }

    pub(crate) fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    pub(crate) fn gravity_and_move(&mut self) {
        if self.x_speed == 0 {
            return;
        }

        if self.velocity < 2.5 {
            self.velocity += 0.5;
        }

        self.y += self.velocity as i32;
        self.x += self.x_speed;

        if self.y < 0 {
            self.y = 0;
        }
    }

    pub(crate) fn speed_powerup(&mut self, score: i32) {
        self.start_score = score;
        self.x_speed += 2;
    }

    pub(crate) fn flap(&mut self) {
        self.velocity = -4.0;
    }
}
