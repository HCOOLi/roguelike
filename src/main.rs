mod camera;
mod combat;
mod creatures;
mod enemy;
mod maptile;
mod movement;
mod player;
mod weapen;

use bevy::{prelude::*, window::WindowResolution};
use camera::CameraPlugin;
use combat::*;
use creatures::DamagePlugin;
use enemy::EnemyPlugin;
use maptile::*;
use movement::MovementPlugin;
use player::PlayerPlugin;
use weapen::WeapenPlugin;

fn main() {
    App::new()
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins((EnemyPlugin, CombatPlugin, DamagePlugin, WeapenPlugin))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new((WIDTH) as f32, (HEIGHT) as f32),
                resizable: false,
                title: String::from("射击游戏"),

                ..default()
            }),
            ..default()
        }))
        .run();
}
