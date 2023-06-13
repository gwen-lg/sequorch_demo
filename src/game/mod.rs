use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup_camera)
			.add_startup_system(setup_test_map);
	}
}

pub fn setup_camera(mut commands: Commands) {
	let mut camera = Camera2dBundle::default();
	camera.projection = OrthographicProjection {
		far: 1000.0,
		//depth_calculation: DepthCalculation::ZDifference,
		//scaling_mode: ScalingMode::FixedHorizontal(5.),
		scale: 0.1,
		..Default::default()
	};
	camera.transform.translation = Vec3::new(128. / 2., 76.8 / 2., 999.0);
	// 1280.0, 768.0

	commands.spawn(camera);
}

pub fn setup_test_map(mut commands: Commands) {
	commands.spawn(SpriteBundle {
		sprite: Sprite {
			custom_size: Some(Vec2::new(1., 75.8)),
			color: Color::rgb(0.9, 0.2, 0.9),
			anchor: Anchor::BottomLeft,
			..Default::default()
		},
		transform: Transform::from_xyz(0. as f32, 1. as f32 / 2., 0.),
		..Default::default()
	});
	commands.spawn(SpriteBundle {
		sprite: Sprite {
			custom_size: Some(Vec2::new(1., 75.8)),
			color: Color::rgb(0.9, 0.2, 0.9),
			anchor: Anchor::BottomLeft,
			..Default::default()
		},
		transform: Transform::from_xyz(127. as f32, 1. as f32, 0.),
		..Default::default()
	});
	commands.spawn(SpriteBundle {
		sprite: Sprite {
			custom_size: Some(Vec2::new(128., 1.)),
			color: Color::rgb(0.2, 0.9, 0.9),
			anchor: Anchor::BottomLeft,
			..Default::default()
		},
		transform: Transform::from_xyz(0. as f32, 0. as f32, 0.),
		..Default::default()
	});
}
