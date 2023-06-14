use bevy::prelude::*;
use bevy::sprite::Anchor;

const NPC_COLOR: Color = Color::rgb(0.8, 0.8, 0.1);
const NPC_SIZE: Vec2 = Vec2::new(2., 2.);

#[derive(Component)]
struct Npc;

pub fn spawn_npc(commands: &mut Commands, transform: Transform) {
	let npc = (
		SpriteBundle {
			sprite: Sprite {
				custom_size: Some(NPC_SIZE),
				color: NPC_COLOR,
				anchor: Anchor::BottomLeft,
				..Default::default()
			},
			transform,
			..Default::default()
		},
		Npc,
	);
	commands.spawn(npc);
}
