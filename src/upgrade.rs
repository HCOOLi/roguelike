use bevy::prelude::*;

#[derive(Component)]
pub struct Grade(pub i32);

#[derive(Component)]
pub struct Expierence(pub i32);

pub struct GradePlugin;

#[derive(Event)]
pub struct AddExpierenceEvent {
    pub value: i32,
}

#[derive(Event)]
pub struct UpgradeEvent {
    pub value: i32,
}

impl Plugin for GradePlugin {
    fn build(&self, app: &mut App) {}
}

fn spawn_grade_ui(mut commands: Commands, query: Query<&Position, &Grade>) {

    
}

fn update_grade_ui() {}
