use std::time::Duration;

use bevy::{
    prelude::{Component, Entity, Vec3},
    reflect::Reflect,
    time::{Timer, TimerMode},
};

#[derive(Component, Default, Reflect)]
pub struct Player {}

#[derive(Component)]
pub struct Foot {}

#[derive(Component)]
pub struct FootTarget {
    pub owner: Entity,
    pub foot: Entity,
    pub anchor: Entity,
    pub pos_offset: Vec3,
}

#[derive(Component)]
pub struct FootPole {
    pub owner: Entity,
    pub pos_offset: Vec3,
}

#[derive(Component)]
pub struct FootAnchor {
    pub foot: Option<Entity>,
    pub current_pos: Vec3,
    pub desired_pos: Vec3,
    pub animation_duration: Duration,
    pub animation_timer: Timer,
    pub pos_error_margin: f32,
    pub max_distance: f32,
    pub moving: bool,
}

impl Default for FootAnchor {
    fn default() -> Self {
        Self {
            foot: None,
            current_pos: Vec3::ZERO,
            desired_pos: Vec3::ZERO,
            animation_duration: Duration::from_secs_f32(0.25),
            animation_timer: Timer::new(Duration::from_secs_f32(0.27), TimerMode::Once),
            pos_error_margin: 0.26,
            max_distance: 0.25,
            moving: false,
        }
    }
}

//Events
pub struct MoveAnchorEvent {
    pub anchor: Entity,
    pub target: Vec3,
}
