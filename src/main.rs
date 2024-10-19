use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::f32::consts::PI;

fn main() {
  App::new()
    .add_plugins((DefaultPlugins, PanOrbitCameraPlugin))
    .add_systems(Startup, startup)
    .run();
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn((
    Camera3dBundle {
      transform: Transform::from_xyz(0.0, 20.0, 75.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
      ..default()
    },
    PanOrbitCamera::default(),
  ));

  commands.spawn(PbrBundle {
    mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)),
    material: materials.add(Color::WHITE),
    ..default()
  });

  commands.spawn(DirectionalLightBundle {
    directional_light: DirectionalLight {
      illuminance: light_consts::lux::OVERCAST_DAY,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform {
      translation: Vec3::new(0.0, 2.0, 0.0),
      rotation: Quat::from_rotation_x(-PI / 4.0),
      ..default()
    },
    ..default()
  });
}
