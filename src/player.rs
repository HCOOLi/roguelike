use crate::creatures::*;
use crate::maptile::*;
use crate::movement::*;
use crate::weapen::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                // player_movement_mouse,
                execute_animations,
                touches,
            ),
        );
    }
}

#[derive(Component)]
pub struct Player;

fn execute_animations(mut query: Query<(&mut TextureAtlas)>) {
    for mut atlas in &mut query {
        if atlas.index == 6 {
            atlas.index = 1;
        } else {
            atlas.index += 1;
        }
    }
}

fn spawn_player(
    mut command: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    command
        .spawn(SpriteBundle {
            texture: texture.clone(),
            transform: Transform::from_scale(Vec3::new(3., 3., 3.)),
            ..default()
        })
        .insert(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 1,
        })
        .insert(Position { x: 0.0, y: 0.0 })
        .insert(Velocity {
            value: 3.0,
            direction: (0.0, 0.0),
        })
        .insert(Player)
        .insert(Health {
            current: 100,
            total: 100,
        })
        .insert(Gun::new(50, 0.5));
}

fn player_movement_keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut v in query.iter_mut() {
        v.direction = (0.0, 0.0);
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            v.direction.0 = -0.1;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            v.direction.0 = 0.1;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            v.direction.1 = -0.1;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            v.direction.1 = 0.1;
        }
    }
}

fn player_movement_mouse(
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&Position, &mut Velocity), With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    for (pos, mut v) in query.iter_mut() {
        v.direction = (0.0, 0.0);
        if mouse.pressed(MouseButton::Left) {
            if let Some(position) = q_windows.single().cursor_position() {
                info!("Cursor is inside the primary window, at {:?}", position);
                v.direction.0 = position.x - (WIDTH / 2) as f32 - pos.x;
                v.direction.1 = -(position.y - (HEIGHT / 2) as f32) - pos.y;
            } else {
                info!("Cursor is not in the game window.");
                return;
            }
        }
    }
}

fn touches(
    mut commands: Commands,
    touches: Res<Touches>,
    mut query: Query<(&Position, &mut Velocity), With<Player>>,
) {
    // There is a lot more information available, see the API docs.
    // This example only shows some very basic things.
    for (_pos, mut v) in query.iter_mut() {
        v.direction = (0.0, 0.0);
        for finger in touches.iter() {
            if touches.just_pressed(finger.id()) {
                info!("A new touch with ID {} just began.", finger.id());
            }
            // commands.spawn(MaterialMesh2dBundle{
            //     mesh

            // })
            // info!(
            //     "Finger {} is at position ({},{}), started from ({},{}).",
            //     finger.id(),
            //     finger.position().x,
            //     finger.position().y,
            //     finger.start_position().x,
            //     finger.start_position().y,
            // );
            v.direction.0 = finger.position().x - finger.start_position().x;
            v.direction.1 = -(finger.position().y - finger.start_position().y);
            info!("v.direction = {:?}", v.direction);
        }
    }
}

#[allow(unused)]
fn player_movement_gamepad() {
    todo!()
}
