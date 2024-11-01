use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::movement::Position;
pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MakeDamage>()
            .add_event::<DespawnEvent>()
            .add_systems(Update, (making_damage, despawn, spawn_health_ui));
    }
}

#[derive(Component)]
pub struct Health {
    pub total: i32,
    pub current: i32,
}

fn spawn_health_ui(
    mut commands: Commands,
    query: Query<(&Position, &Health)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rectangle_mesh = Mesh2dHandle(meshes.add(Rectangle {
        half_size: Vec2 { x: 20., y: 1. },
    }));

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: rectangle_mesh,
            material: materials.add(ColorMaterial::from_color(Color::srgb(1., 0., 0.))),
            ..default()
        })
        .insert(Text2dBundle {
            text: Text::from_section(
                format!("hp:{}/{}", 20, 100),
                TextStyle {
                    font_size: 20.0,
                    color: Srgba::WHITE.into(),
                    ..default()
                },
            )
            .with_justify(JustifyText::Left),
            ..default()
        });
}

#[derive(Event, Clone)]
pub struct MakeDamage {
    pub object: Entity,
    pub damage: i32,
}

#[derive(Event)]
pub struct DespawnEvent {
    pub object: Entity,
}

fn despawn(mut commands: Commands, mut event_reader: EventReader<DespawnEvent>) {
    for dse in event_reader.read() {
        if let Some(mut entity) = commands.get_entity(dse.object) {
            entity.despawn();
        }
    }
}

fn making_damage(
    mut enemy: Query<&mut Health>,
    mut event_reader: EventReader<MakeDamage>,
    mut event_writer: EventWriter<DespawnEvent>,
) {
    for make_damage in event_reader.read() {
        if let Ok(ref mut h) = enemy.get_mut(make_damage.object) {
            h.current -= make_damage.damage;
            if h.current <= 0 {
                event_writer.send(DespawnEvent {
                    object: make_damage.object,
                });
            }
        }
    }
}
