use crate::effects::{Effect, GravityBuffEffect, SpeedBuffEffect};
use bracket_lib::color::{BLACK, BLUE, ORANGE, WHITE};
use bracket_lib::prelude::{to_cp437, BTerm, RandomNumberGenerator};

use crate::player::*;
use crate::SCREEN_HEIGHT;

pub trait PowerUp {
    fn render(&self, ctx: &mut BTerm, player_x: i32);
    fn is_collide(&self, player: &mut Player) -> bool;
    fn get_x(&self) -> i32;
    fn get_effect(&self) -> Box<dyn Effect>;
}

pub struct SpeedPowerUp {
    pub(crate) x: i32,
    y: i32,
    size: i32,
}

impl SpeedPowerUp {
    pub(crate) fn new(x: i32, y: i32, mut rng: RandomNumberGenerator) -> SpeedPowerUp {
        let size = rng.range(5, 40);
        let mut side_move = rng.range(0, SCREEN_HEIGHT / 2 - size / 2 - 1);
        let side_changer = rng.range(0, 2);

        if side_changer == 1 {
            side_move *= -1;
        }

        SpeedPowerUp {
            x,
            y: y / 2 - size / 2 + side_move,
            size,
        }
    }
}

impl PowerUp for SpeedPowerUp {
    fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;

        for i in 0..self.size {
            ctx.set(screen_x, self.y + i, WHITE, BLACK, to_cp437('>'));
        }
    }

    fn is_collide(&self, player: &mut Player) -> bool {
        let mut hit = false;

        if self.x == player.x && (self.y..self.y + self.size).contains(&player.y) {
            hit = true;
        }
        hit
    }

    fn get_effect(&self) -> Box<dyn Effect> {
        Box::new(SpeedBuffEffect::new())
    }

    fn get_x(&self) -> i32 {
        self.x
    }
}

pub struct GravityDebuff {
    pub(crate) x: i32,
    y: i32,
    size: i32,
}

impl GravityDebuff {
    pub(crate) fn new(x: i32, y: i32, mut rng: RandomNumberGenerator) -> GravityDebuff {
        let size = rng.range(5, 40);
        let mut side_move = rng.range(0, SCREEN_HEIGHT / 2 - size / 2 - 1);
        let side_changer = rng.range(0, 2);

        if side_changer == 1 {
            side_move *= -1;
        }

        GravityDebuff {
            x,
            y: y / 2 - size / 2 + side_move,
            size,
        }
    }
}
impl PowerUp for GravityDebuff {
    fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;

        for i in 0..self.size {
            ctx.set(screen_x, self.y + i, ORANGE, BLUE, to_cp437('V'));
        }
    }

    fn is_collide(&self, player: &mut Player) -> bool {
        let mut hit = false;

        if self.x == player.x && (self.y..self.y + self.size).contains(&player.y) {
            hit = true;
        }
        hit
    }

    fn get_effect(&self) -> Box<dyn Effect> {
        Box::new(GravityBuffEffect::new())
    }

    fn get_x(&self) -> i32 {
        self.x
    }
}
