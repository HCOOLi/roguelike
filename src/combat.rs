use crate::creatures::*;
use crate::enemy::*;
use crate::movement::*;
use crate::player::*;
use crate::weapen::*;
use bevy::prelude::*;

use rand::{thread_rng, Rng};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            3.5,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (spawn_enemy, shoot, enemy_velocity_update, collision_detect),
        );
    }
}

pub fn spawn_enemy(
    mut command: Commands,
    asset_server: ResMut<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    let mut rng = thread_rng();
    for _ in 0..3 {
        let (x, y) = (rng.gen_range(-100.0..100.0), rng.gen_range(-250.0..250.0));
        command
            .spawn(SpriteBundle {
                texture: asset_server.load("enemy/zombie_head.png"),
                ..Default::default()
            })
            .insert(Position {
                x: x as f32,
                y: y as f32,
            })
            .insert(Velocity {
                value: 0.5,
                direction: (0.0, 0.0),
            })
            .insert(Enemy)
            .insert(Health {
                current: 100,
                total: 100,
            });
    }
}
//  一直追向玩家
pub fn enemy_velocity_update(
    mut enemy: Query<(&Position, &mut Velocity), With<Enemy>>,
    player: Query<&Position, (With<Player>, Without<Enemy>)>,
) {
    let player_pos = player.get_single().unwrap();

    enemy.iter_mut().for_each(|(e_pos, mut e_v)| {
        e_v.direction = (player_pos.x - e_pos.x, player_pos.y - e_pos.y)
    });
}

fn shoot(
    mut command: Commands,
    time: Res<Time>,
    asset_server: ResMut<AssetServer>,
    enemy: Query<&Position, With<Enemy>>,
    mut player: Query<(&Position, &mut Gun), Without<Enemy>>,
) {
    let (player_pos, mut gun) = player.get_single_mut().unwrap();
    if !gun.cold_down.tick(time.delta()).finished() {
        return;
    }
    let mut min: f32 = 9999.0;
    let mut min_pos = Position { x: 0.0, y: 0.0 };
    enemy.iter().for_each(|e_pos| {
        let x = player_pos.distance(e_pos);
        if x < min {
            min = x;
            min_pos = *e_pos;
        }
    });
    if min == 9999.0 {
        return;
    }
    command
        .spawn(SpriteBundle {
            texture: asset_server.load("fish.png"),
            transform: Transform {
                scale: Vec3::new(0.1, 0.1, 0.1),
                ..default()
            },
            ..default()
        })
        .insert(player_pos.clone())
        .insert(Velocity {
            value: gun.speed,
            direction: (min_pos.x - player_pos.x, min_pos.y - player_pos.y),
        })
        .insert(gun.gen_bullet());
}

fn collision_detect(
    enemy: Query<(Entity, &Position), With<Enemy>>,
    bullet: Query<(Entity, &Position, &Bullet)>,
    mut event_writer: EventWriter<MakeDamage>,
    mut despawn_event: EventWriter<DespawnEvent>,
) {
    for (e_entity, pe) in enemy.iter() {
        for (b_entity, pb, b) in bullet.iter() {
            if pe.distance(pb) < 10.0 {
                event_writer.send(MakeDamage {
                    object: e_entity,
                    damage: b.damage,
                });
                despawn_event.send(DespawnEvent { object: b_entity });
            }
        }
    }
}
