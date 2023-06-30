use bevy::prelude::*;
use bevy::{ecs::system::EntityCommands, sprite::Anchor};

#[derive(Component)]
struct Floor;

const FLOOR_COLOR: Color = Color::rgb(0.2, 0.9, 0.9);

pub fn setup_test_map(mut commands: Commands) {
	let mut map_parent = commands.spawn(SpatialBundle::default());
	floor(
		&mut map_parent,
		Vec2::new(1., 75.8),
		Transform::from_xyz(0., 1. / 2., 0.),
	);
	floor(
		&mut map_parent,
		Vec2::new(1., 75.8),
		Transform::from_xyz(127., 1., 0.),
	);
	floor(
		&mut map_parent,
		Vec2::new(128., 1.),
		Transform::from_xyz(0., 0., 0.),
	);
}

fn floor(commands: &mut EntityCommands, size: Vec2, transform: Transform) {
	let sprite = (
		SpriteBundle {
			sprite: Sprite {
				custom_size: Some(size),
				color: FLOOR_COLOR,
				anchor: Anchor::BottomLeft,
				..Default::default()
			},
			transform,
			..Default::default()
		},
		Floor,
	);
	commands.with_children(|parent| {
		parent.spawn(sprite);
	});
}
