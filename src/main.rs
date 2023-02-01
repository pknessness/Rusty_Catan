use bevy::prelude::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.3,0.2,0.4)))
    .add_startup_system(spawn_camera)
    .add_plugins(DefaultPlugins)
    .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.2,0.1,0.3).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}