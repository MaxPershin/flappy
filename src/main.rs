use bracket_lib::prelude::*;
use crate::GameMode::Menu;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 0.0;

struct SpeedPowerUp {
    x: i32,
    y: i32,
    size: i32,
}

impl SpeedPowerUp {
    fn new(x: i32, y: i32, size: i32) -> SpeedPowerUp {
        SpeedPowerUp {
            x,
            y,
            size,
        }
    }

    fn render(&self, ctx: &mut BTerm, player_x: i32){
        let screen_x = self.x - player_x;

        for i in 0..self.size {
            ctx.set(screen_x,
                    self.y + i,
                    WHITE,
                    BLACK,
                    to_cp437('>'),
            );
        }

    }

    fn hit_speed_power_up(&self, player: &mut Player) -> bool {

        let mut hit = false;

        if self.x == player.x {
            if (self.y..self.y+self.size).contains(&player.y){
                hit = true;
            }
        }
        hit
    }
}

struct Obstacle {
    x: i32,
    y_gap: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Obstacle {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            y_gap: random.range(20, 40),
            size: i32::max(2, 20-score/4),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.y_gap - half_size {
            for i in 0..3 {
                ctx.set(screen_x+i,
                        y,
                        GREEN,
                        YELLOW,
                        to_cp437('|'),
                );
            }
        }

        for y in self.y_gap + half_size..SCREEN_HEIGHT {
            for i in 0..3 {
                ctx.set(screen_x+i,
                        y,
                        GREEN,
                        YELLOW,
                        to_cp437('|'),
                );
            }
        }
    }
    fn hit_obstacle(&mut self, player: &Player) -> bool {
        let half_size = self.size / 2;

        let mut does_x_match = false;

        for i in 0..3 {
            if self.x+i == player.x {
                does_x_match = true;
            }
        }

        let is_above_gap = player.y < self.y_gap - half_size;
        let is_below_gap = player.y > self.y_gap + half_size;

        does_x_match && (is_above_gap || is_below_gap)
    }

}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
    x_speed: i32,
    start_score: i32,
}

impl Player {
    fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y,
            velocity: 0.0,
            x_speed: 0,
            start_score: 0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0,
                self.y,
                YELLOW,
                BLACK,
                to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {

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

    fn speed_powerup(&mut self, score: i32){
        self.start_score = score;
        self.x_speed += 2;
    }

    fn flap(&mut self) {
        self.velocity = -4.0;
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    player: Player,
    obstacle: Obstacle,
    speed_power_up: SpeedPowerUp,
    frame_time: f32,
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> State {
        State {
            player: Player::new(5, 25),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            speed_power_up: SpeedPowerUp::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 0),
            frame_time: 0.0,
            mode: Menu,
            score: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        if self.player.x_speed > 1 {
            ctx.cls_bg(RED);
        } else {
            ctx.cls_bg(NAVYBLUE)
        }
        ctx.print(0, 0, "Press SPACE to FLY!");
        ctx.print(0, 1, &format!("Your score: {}", self.score));

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => {
                    if self.player.x_speed == 0 {
                        self.player.x_speed = 1;
                    }
                    self.player.flap();
                },
                _ => {}
            }
        }

        //Render chain
        self.player.render(ctx);
        self.obstacle.render(ctx, self.player.x);
        self.speed_power_up.render(ctx, self.player.x);

        //Next objects chain
        if self.player.x > self.obstacle.x+3 {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        //Create new speed buff if chance
        let mut random = RandomNumberGenerator::new();
        let chance = random.range(0, 300);

        if (0..1).contains(&chance) {
            if self.player.x > self.speed_power_up.x {
                let size = random.range(5, 40);
                let mut side_move = random.range(0, SCREEN_HEIGHT/2-size/2-1);
                let side_changer = random.range(0, 2);

                if side_changer == 1 {
                    side_move *= -1;
                }

            self.speed_power_up = SpeedPowerUp::new(self.player.x + SCREEN_WIDTH, SCREEN_HEIGHT/2-size/2+side_move, size);

        }}

        if self.score - self.player.start_score == 10 {
            self.player.start_score = 0;
            self.player.x_speed = 1;
        }

        //Check collisions
        if self.speed_power_up.hit_speed_power_up(&mut self.player){
            self.player.speed_powerup(self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }

    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, self.score);

        let mut random = RandomNumberGenerator::new();
        let size = random.range(5, 40);
        let mut side_move = random.range(0, SCREEN_HEIGHT/2-size/2-1);
        let side_changer = random.range(0, 2);

        if side_changer == 1 {
            side_move *= -1;
        }

        self.speed_power_up = SpeedPowerUp::new(self.player.x + SCREEN_WIDTH, SCREEN_HEIGHT/2-size/2+side_move, size);

        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm){
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

    fn dead(&mut self, ctx: &mut BTerm){
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

fn main() -> BError{

    let context = BTermBuilder::simple80x50()
        .with_title("FlappyDragonGame v0.1.1")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())

}
