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
    obstacle: Obstacle,
    speed_power_up: SpeedPowerUp,
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> State {
        State {
            player: Player::new(5, 25),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            speed_power_up: SpeedPowerUp::new(
                SCREEN_WIDTH / 2,
                SCREEN_HEIGHT / 2,
                RandomNumberGenerator::new(),
            ),
            mode: Menu,
            score: 0,
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
                VirtualKeyCode::Q => match self.mode {
                    GameMode::Playing => {
                        self.mode = GameMode::End;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn render_chain(&mut self, ctx: &mut BTerm) {
        self.player.render(ctx);
        self.obstacle.render(ctx, self.player.x);
        self.speed_power_up.render(ctx, self.player.x);
    }

    fn objects_spawn(&mut self) {
        //Creating next obstacle
        if self.player.x > self.obstacle.x + 3 {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        //Creating new speed buff
        let mut random = RandomNumberGenerator::new();
        let chance = random.range(0, 300);

        if (0..1).contains(&chance) {
            if self.player.x > self.speed_power_up.x {
                self.speed_power_up =
                    SpeedPowerUp::new(self.player.x + SCREEN_WIDTH, SCREEN_HEIGHT, random);
            }
        }
    }

    fn collisions_check(&mut self) {
        //Check collisions
        if self.speed_power_up.hit_speed_power_up(&mut self.player) {
            self.player.speed_powerup(self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.draw_background(ctx);
        self.draw_overlay(ctx);

        self.move_and_gravity();
        self.listen_keys(ctx);

        self.render_chain(ctx);
        self.objects_spawn();

        if self.score - self.player.start_score == 10 {
            self.player.start_score = 0;
            self.player.x_speed = 1;
        }

        self.collisions_check();
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, self.score);
        self.speed_power_up = SpeedPowerUp::new(
            self.player.x + SCREEN_WIDTH,
            SCREEN_HEIGHT,
            RandomNumberGenerator::new(),
        );
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
