mod effects;
mod obstacle;
mod player;
mod powerups;

use crate::obstacle::*;
use crate::player::*;
use crate::powerups::*;

use crate::GameMode::Menu;
use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    player: Player,
    obstacle_vec: Vec<Obstacle>,
    powerups: Vec<Box<dyn PowerUp>>,
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> State {
        State {
            player: Player::new(5, 25),
            obstacle_vec: vec![],
            powerups: vec![],
            mode: Menu,
            score: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.draw_background(ctx);
        self.draw_overlay(ctx);

        self.move_and_gravity();
        self.listen_keys(ctx);

        self.render_chain(ctx);
        self.objects_spawn();

        self.collisions_check();
        self.out_of_vision_check();
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.score = 0;
        self.obstacle_vec = vec![];
        self.powerups = vec![];
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Start Game");
        ctx.print_centered(9, "(Q) Quit game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "GAME OVER - YOU ARE DEAD!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));

        ctx.print_centered(8, "(P) Restart Game");
        ctx.print_centered(9, "(Q) Quit game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn update_effects(&mut self) {
        self.player.apply_effects();
    }

    fn out_of_vision_check(&mut self) {
        let mut need_to_update_effects = false;
        let mut helper_vec = vec![];

        for (i, obstacle) in self.obstacle_vec.iter().enumerate() {
            if obstacle.x + 3 < self.player.x {
                helper_vec.push(i);
                need_to_update_effects = true;
            }
        }

        if need_to_update_effects {
            self.update_effects();
        }

        for i in &helper_vec {
            self.obstacle_vec.remove(*i);
            self.score += 1;
        }

        helper_vec.clear();

        for (i, powerup) in &mut self.powerups.iter().enumerate() {
            if powerup.get_x() < self.player.x {
                helper_vec.push(i);
            }
        }

        for i in helper_vec {
            self.powerups.remove(i);
        }
    }

    fn draw_background(&mut self, ctx: &mut BTerm) {
        if self.player.x_speed > 1 {
            ctx.cls_bg(RED);
        } else {
            ctx.cls_bg(NAVYBLUE)
        }
    }

    fn draw_overlay(&self, ctx: &mut BTerm) {
        ctx.print(0, 0, "Press SPACE to FLY!");
        ctx.print(0, 1, &format!("Your score: {}", self.score));
    }

    fn move_and_gravity(&mut self) {
        self.player.gravity_and_move();
    }

    fn listen_keys(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => {
                    if self.player.x_speed == 0 {
                        self.player.x_speed = 1;
                    }
                    self.player.flap();
                }
                VirtualKeyCode::Q => {
                    if let GameMode::Playing = self.mode {
                        self.mode = GameMode::End;
                    }
                }
                _ => {}
            }
        }
    }

    fn render_chain(&mut self, ctx: &mut BTerm) {
        self.player.render(ctx);

        for obstacle in &mut self.obstacle_vec {
            obstacle.render(ctx, self.player.x);
        }

        for powerup in &mut self.powerups {
            powerup.render(ctx, self.player.x);
        }
    }

    fn objects_spawn(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        let chance = rng.range(0, 300);
        let buff_choice = rng.range(0, 100);

        //Creating OBSTACLE
        if self.obstacle_vec.is_empty() {
            self.obstacle_vec
                .push(Obstacle::new(self.player.x + SCREEN_WIDTH, self.score));
        }

        if self.obstacle_vec.len() < 3 {
            for i in 1..4 {
                self.obstacle_vec.push(Obstacle::new(
                    self.player.x + SCREEN_WIDTH + SCREEN_WIDTH * i / 2,
                    self.score,
                ));
            }
        }
        //Creating SPEED POWERUP
        if (0..1).contains(&chance) {
            if buff_choice >= 50 {
                self.powerups.push(Box::new(SpeedPowerUp::new(
                    self.player.x + SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    rng,
                )));
            } else {
                self.powerups.push(Box::new(GravityDebuff::new(
                    self.player.x + SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    rng,
                )));
            }
        }
    }

    fn collisions_check(&mut self) {
        //Check collisions
        for powerup in &mut self.powerups {
            if powerup.is_collide(&mut self.player) {
                let new_effect = powerup.get_effect();
                new_effect.apply(&mut self.player);
                self.player.effects.push(new_effect);
            }
        }

        for obstacle in &mut self.obstacle_vec {
            if obstacle.hit_obstacle(&self.player) {
                self.mode = GameMode::End;
                break;
            }
        }

        if self.player.y > SCREEN_HEIGHT || self.player.y < 0 {
            self.mode = GameMode::End;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::End => self.dead(ctx),
            Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("FlappyDragonGame v0.1.1")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
