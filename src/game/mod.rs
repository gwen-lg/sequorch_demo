mod door;
mod level;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup_camera)
			.add_startup_system(level::setup_test_map)
			.add_startup_system(setup_doors);
	}
}

const VIEWPORT_CAM_VIEW: Vec2 = Vec2::new(1280., 768.);

pub fn setup_camera(mut commands: Commands) {
	let mut camera = Camera2dBundle::default();
	camera.projection = OrthographicProjection {
		far: 1000.0,
		scale: 0.1,
		..Default::default()
	};
	camera.transform.translation =
		Vec3::new(VIEWPORT_CAM_VIEW.x / 20., VIEWPORT_CAM_VIEW.y / 20., 999.0);

	commands.spawn(camera);
}

pub fn setup_doors(mut commands: Commands) {
	const DOOR_SIZE: Vec2 = Vec2::new(10., 10.);
	door::spawn_door(&mut commands, DOOR_SIZE, Transform::from_xyz(10., 1.0, 0.));
	door::spawn_door(&mut commands, DOOR_SIZE, Transform::from_xyz(50., 1.0, 0.));
	door::spawn_door(&mut commands, DOOR_SIZE, Transform::from_xyz(105., 1.0, 0.));
}
