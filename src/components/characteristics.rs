use std::{ops::Range, time::Duration};

use bevy::prelude::*;

pub trait Characteristics {
    fn current_level(&self) -> usize;
    fn level_up_costs(&self) -> Vec<(Range<usize>, usize)>;
    fn level_up(&mut self);
}

#[derive(Component)]
pub struct AttackSpeed {
    level: usize,
    attacks_per_second: f32,
    timer: Timer,
}

impl AttackSpeed {
    pub fn new(attack_per_second: f32) -> Self {
        Self {
            attacks_per_second: attack_per_second,
            level: 1,
            timer: Timer::from_seconds(1. / attack_per_second, TimerMode::Once),
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        self.timer.tick(dt);
    }

    pub fn can_trigger(&self) -> bool {
        self.timer.finished()
    }

    pub fn try_trigger(&mut self) -> Option<()> {
        self.can_trigger().then(|| self.timer.reset())
    }

    pub fn attack_per_second(&self) -> f32 {
        self.attacks_per_second
    }
}

impl Characteristics for AttackSpeed {
    fn current_level(&self) -> usize {
        self.level
    }

    fn level_up_costs(&self) -> Vec<(Range<usize>, usize)> {
        vec![(1..10, 1), (10..20, 2)]
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.attacks_per_second += 0.5;
       
        self.timer.set_duration(Duration::from_secs_f32(1. / self.attacks_per_second));

    }
}
