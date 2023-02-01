use bevy::prelude::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.3,0.2,0.4)))
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_scene)
    .add_plugins(DefaultPlugins)
    .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.2,1.1,4.3).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_scene(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {size: 5.0})),
        material: materials.add(Color::rgb(0.6,0.2,0.8).into()),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
        material: materials.add(Color::rgb(0.6,0.2,0.8).into()),
        transform: Transform::from_xyz(0.0,0.5,0.0),
        ..default()
    });
    
}