use bevy::prelude::*;

pub const WIDTH: i32 = 1080 / 3;
pub const HEIGHT: i32 = 2340 / 3;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    const PIC: i32 = 64;
    for x in 0..(WIDTH / PIC + 1) {
        for y in 0..(HEIGHT / PIC + 1) {
            commands.spawn(SpriteBundle {
                texture: asset_server.load("wood_red.png"),
                transform: Transform {
                    translation: Vec3 {
                        x: (x * PIC - WIDTH / 2) as f32,
                        y: (y * PIC - HEIGHT / 2) as f32,
                        z: -10.0,
                    },
                    ..default()
                },
                ..default()
            });
        }
    }
}
