use crate::effects::Effect;
use bracket_lib::color::{BLACK, YELLOW};
use bracket_lib::prelude::{to_cp437, BTerm};

pub struct Player {
    pub(crate) x: i32,
    pub(crate) y: i32,
    velocity: f32,
    pub velocity_base: f32,
    pub(crate) x_speed: i32,
    pub(crate) effects: Vec<Box<dyn Effect>>,
}

impl Player {
    pub(crate) fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y,
            velocity: 0.0,
            velocity_base: 0.5,
            x_speed: 0,
            effects: vec![],
        }
    }

    pub fn apply_effects(&mut self) {
        let mut helper_vec = vec![];

        for (i, effect) in self.effects.iter_mut().enumerate() {
            effect.tick_clocks();
            if effect.is_finished() {
                helper_vec.push(i);
            }
        }

        for i in helper_vec {
            let x = self.effects.remove(i);
            x.release(self);
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
            self.velocity += self.velocity_base;
        }

        self.y += self.velocity as i32;
        self.x += self.x_speed;
    }

    pub(crate) fn flap(&mut self) {
        self.velocity = -4.0;
    }
}
