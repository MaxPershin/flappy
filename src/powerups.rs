use bracket_lib::color::{BLACK, WHITE};
use bracket_lib::prelude::{to_cp437, BTerm, RandomNumberGenerator};

use crate::player::*;
use crate::SCREEN_HEIGHT;

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

    pub(crate) fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;

        for i in 0..self.size {
            ctx.set(screen_x, self.y + i, WHITE, BLACK, to_cp437('>'));
        }
    }

    pub(crate) fn hit_speed_power_up(&self, player: &mut Player) -> bool {
        let mut hit = false;

        if self.x == player.x {
            if (self.y..self.y + self.size).contains(&player.y) {
                hit = true;
            }
        }
        hit
    }
}
