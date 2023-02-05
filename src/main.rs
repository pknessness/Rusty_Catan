use bevy::prelude::*;
use bevy_vox_mesh::VoxMeshPlugin;
use std::f32::consts::PI;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.3,0.2,0.4)))
    .add_plugins(DefaultPlugins)
    .add_plugin(VoxMeshPlugin::default())
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_scene)
    .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.2,5.1,3.3).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_scene(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {size: 40.0})),
        material: materials.add(Color::rgb(0.1,0.2,0.8).into()),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
    //     material: materials.add(Color::rgb(0.6,0.2,0.8).into()),
    //     transform: Transform::from_xyz(0.0,0.5,0.0),
    //     ..default()
    // });
    
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
    //     material: materials.add(Color::rgb(0.6,0.2,0.8).into()),
    //     transform: Transform::from_xyz(0.0,0.5,0.0),
    //     ..default()
    // });
    commands.spawn(PbrBundle {
        transform: Transform::from_scale((0.1, 0.1, 0.1).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)) * Transform::from_xyz(0.0,-3.9,0.0),
        mesh: assets.load("MountainHex1.vox"),
        material: materials.add(Color::rgb(1., 1., 1.).into()),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_scale((0.1, 0.1, 0.1).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)) * Transform::from_xyz(-28.0,-3.9,0.0),
        mesh: assets.load("BrickHex1.vox"),
        material: materials.add(Color::rgb(1., 1., 1.).into()),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_scale((0.1, 0.1, 0.1).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)) * Transform::from_xyz(-28.0,-3.9,-48.0),
        mesh: assets.load("WheatHex1.vox"),
        material: materials.add(Color::rgb(1., 1., 1.).into()),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_scale((0.1, 0.1, 0.1).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)) * Transform::from_xyz(-14.0,-3.9,-24.0),
        mesh: assets.load("PlainsHex1.vox"),
        material: materials.add(Color::rgb(1., 1., 1.).into()),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        transform: Transform::from_scale((0.1, 0.1, 0.1).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)) * Transform::from_xyz(-42.0,-3.9,-24.0),
        mesh: assets.load("ForestHex1.vox"),
        material: materials.add(Color::rgb(1., 1., 1.).into()),
        ..Default::default()
    });

}

// use bevy::prelude::*;
// use bevy_vox_mesh::VoxMeshPlugin;
// use std::f32::consts::PI;

// fn main() {
//     App::default()
//         .add_plugins(DefaultPlugins)
//         .add_plugin(VoxMeshPlugin::default())
//         .add_startup_system(setup)
//         .run();
// }

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut stdmats: ResMut<Assets<StandardMaterial>>,
//     assets: Res<AssetServer>,
// ) {
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..Default::default()
//     });

//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..default()
//     });

//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
//         material: stdmats.add(Color::rgb(0.3, 0.5, 0.3).into()),
//         ..Default::default()
//     });

//     commands.spawn(PbrBundle {
//         transform: Transform::from_scale((0.01, 0.01, 0.01).into())
//             * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
//         mesh: assets.load("chicken.vox"),
//         material: stdmats.add(Color::rgb(1., 1., 1.).into()),
//         ..Default::default()
//     });
// }