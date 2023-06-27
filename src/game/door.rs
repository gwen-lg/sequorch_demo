use bevy::prelude::*;
use bevy::sprite::Anchor;

const DOOR_COLOR: Color = Color::rgb(0.2, 0.2, 0.9);

#[derive(Component)]
pub struct Door;

pub fn spawn_door(commands: &mut Commands, size: Vec2, transform: Transform) {
	let door = (
		SpriteBundle {
			sprite: Sprite {
				custom_size: Some(size),
				color: DOOR_COLOR,
				anchor: Anchor::BottomLeft,
				..Default::default()
			},
			transform,
			..Default::default()
		},
		Door,
	);
	commands.spawn(door);
}
