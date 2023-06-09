mod map;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup_camera)
			.add_startup_system(map::spawn_floor);
	}
}

pub fn setup_camera(mut commands: Commands) {
	let mut camera = Camera2dBundle::default();
	camera.projection = OrthographicProjection {
		far: 1000.0,
		//depth_calculation: DepthCalculation::ZDifference,
		scaling_mode: ScalingMode::FixedHorizontal(5.),
		..Default::default()
	};
	camera.transform.scale = Vec3::new(10., 10., 1.);
	camera.transform.translation = Vec3::new(25., 15., 1000.);

	commands.spawn(camera);
}
