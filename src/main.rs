use bevy::prelude::*;

fn main() {
  let mut app = App::new();
  app
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Repro".to_string(),
            ..default()
          }),
          ..default()
        })
    );

  app
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn((
    Camera3d::default(),
    Camera::default(),
    Transform::default(),
    bevy::core_pipeline::tonemapping::Tonemapping::None,
  ));
}
