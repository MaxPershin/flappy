use bracket_lib::color::{GREEN, YELLOW};
use bracket_lib::prelude::{to_cp437, BTerm, RandomNumberGenerator};

use crate::player::*;
use crate::SCREEN_HEIGHT;

pub struct Obstacle {
    pub(crate) x: i32,
    y_gap: i32,
    size: i32,
}

impl Obstacle {
    pub(crate) fn new(x: i32, score: i32) -> Obstacle {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            y_gap: random.range(20, 40),
            size: i32::max(2, 20 - score / 4),
        }
    }

    pub(crate) fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.y_gap - half_size {
            for i in 0..3 {
                ctx.set(screen_x + i, y, GREEN, YELLOW, to_cp437('|'));
            }
        }

        for y in self.y_gap + half_size..SCREEN_HEIGHT {
            for i in 0..3 {
                ctx.set(screen_x + i, y, GREEN, YELLOW, to_cp437('|'));
            }
        }
    }
    pub(crate) fn hit_obstacle(&mut self, player: &Player) -> bool {
        let half_size = self.size / 2;

        let mut does_x_match = false;

        for i in 0..3 {
            if self.x + i == player.x {
                does_x_match = true;
            }
        }

        let is_above_gap = player.y < self.y_gap - half_size;
        let is_below_gap = player.y > self.y_gap + half_size;

        does_x_match && (is_above_gap || is_below_gap)
    }
}
