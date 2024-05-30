use crate::player::Player;

pub trait Effect {
    fn tick_clocks(&mut self);
    fn apply(&self, player: &mut Player);
    fn release(&self, player: &mut Player);
    fn is_finished(&self) -> bool;
}

pub struct SpeedBuffEffect {
    time: i32,
}

impl SpeedBuffEffect {
    pub fn new() -> SpeedBuffEffect {
        SpeedBuffEffect { time: 10 }
    }
}

impl Effect for SpeedBuffEffect {
    fn tick_clocks(&mut self) {
        self.time -= 1;
    }

    fn apply(&self, player: &mut Player) {
        player.x_speed += 1;
    }

    fn release(&self, player: &mut Player) {
        player.x_speed -= 1;
    }

    fn is_finished(&self) -> bool {
        self.time == 0
    }
}

pub struct GravityBuffEffect {
    time: i32,
}

impl GravityBuffEffect {
    pub fn new() -> GravityBuffEffect {
        GravityBuffEffect { time: 5 }
    }
}

impl Effect for GravityBuffEffect {
    fn tick_clocks(&mut self) {
        self.time -= 1;
    }

    fn apply(&self, player: &mut Player) {
        player.velocity_base += 0.5;
    }

    fn release(&self, player: &mut Player) {
        player.velocity_base -= 0.5;
    }

    fn is_finished(&self) -> bool {
        self.time == 0
    }
}
