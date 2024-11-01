use crate::maptile::*;
#[allow(unused)]
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn distance(&self, p2: &Position) -> f32 {
        f32::sqrt((self.x - p2.x) * (self.x - p2.x) + (self.y - p2.y) * (self.y - p2.y))
    }
    pub fn update_by_velocity(&mut self, v: &Velocity) {
        let (vx, vy) = v.uni();
        self.x += vx;
        if self.x < -(WIDTH / 2) as f32 {
            self.x = -(WIDTH / 2) as f32;
        }
        if self.x > (WIDTH / 2) as f32 {
            self.x = (WIDTH / 2) as f32;
        }
        self.y += vy;
    }
}

#[derive(Component)]
pub struct Velocity {
    pub value: f32,
    pub direction: (f32, f32),
}

#[derive(Bundle)]
struct MovementBundle {
    pub position: Position,
    pub velocity: Velocity,
}

impl Velocity {
    // 防止斜线导致速度变 sqrt2 倍
    pub fn uni(&self) -> (f32, f32) {
        let dis =
            f32::sqrt(self.direction.0 * self.direction.0 + self.direction.1 * self.direction.1);
        if self.value == 0.0 || dis == 0.0 {
            return (0.0, 0.0);
        }

        (
            self.value / dis * self.direction.0,
            self.value / dis * self.direction.1,
        )
    }

    pub fn unify(&mut self) {
        let dis =
            f32::sqrt(self.direction.0 * self.direction.0 + self.direction.1 * self.direction.1);
        if self.value == 0.0 || dis == 0.0 {
            self.direction = (0.0, 0.0);
        }

        self.direction = (
            self.value / dis * self.direction.0,
            self.value / dis * self.direction.1,
        );
    }
}

// TODO: if have convas
pub fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(pos.x, pos.y, transform.translation.z);
    }
}

// TODO: add time;
pub fn position_update(mut q: Query<(&mut Position, &Velocity)>) {
    for (mut pos, v) in q.iter_mut() {
        pos.update_by_velocity(v);
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (position_translation, position_update));
    }
}
