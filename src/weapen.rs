use bevy::prelude::*;

pub struct WeapenPlugin;

impl Plugin for WeapenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_bullet);
    }
}

#[derive(Component)]
pub struct Bullet {
    pub damage: i32,
    pub life_time: Timer,
}

impl Bullet {
    pub fn new(damage: i32, lt: f32) -> Self {
        Bullet {
            damage,
            life_time: Timer::from_seconds(lt, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct Gun {
    pub speed: f32,
    pub damage: i32,
    pub cold_down: Timer,
}

impl Gun {
    pub fn new(damage: i32, cd: f32) -> Self {
        Gun {
            speed: 5.0,
            damage,
            cold_down: Timer::from_seconds(cd, TimerMode::Repeating),
        }
    }

    pub fn gen_bullet(&self) -> Bullet {
        Bullet::new(self.damage, 3.0)
    }
}

fn despawn_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bullet)>,
) {
    for (e, mut bullet) in query.iter_mut() {
        if bullet.life_time.tick(time.delta()).finished() {
            commands.entity(e).despawn();
        }
    }
}
