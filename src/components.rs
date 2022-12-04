use std::time::Duration;

use bevy::{
    prelude::{Component, Entity, ReflectComponent, Vec3},
    reflect::Reflect,
    time::{Timer, TimerMode},
};

/**
 * Using some fields with gui inspector for debugging purposes
 * TODO: move to a different componenet
 */
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub current_speed: Vec3,
    pub move_speed: f32,
    pub rotate_speed: f32,
    pub grounded: bool,
    pub jumping: bool,
    pub jump_power: f32,
    pub jump_time: f32,
    pub jump_time_max: f32,
    //move
    pub walk_height: f32,
    pub walk_width: f32,
    pub walk_spread: Vec3,
    pub pole_offset: Vec3,
    pub pole_spread: Vec3,
}
#[derive(Component)]
pub struct Ground {}

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
    pub target: Option<Entity>,
    pub animation_duration: Duration,
    pub animation_timer: Timer,
    pub pos_error_margin: f32,
    pub max_distance: f32,
    pub moving: bool,
    pub inverted: bool,
}

impl Default for FootAnchor {
    fn default() -> Self {
        Self {
            foot: None,
            target: None,
            animation_duration: Duration::from_secs_f32(0.2),
            animation_timer: Timer::new(Duration::from_secs_f32(0.2), TimerMode::Once),
            pos_error_margin: 0.16,
            max_distance: 0.55,
            moving: false,
            inverted: false,
        }
    }
}

//Events
pub struct MoveAnchorEvent {
    pub anchor: Entity,
    pub target: Entity,
    pub animation_duration: Duration,
}
