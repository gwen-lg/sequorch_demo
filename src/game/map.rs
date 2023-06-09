use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

const LEVEL_SIZE: usize = 150;

pub fn spawn_floor(mut commands: Commands) {
	let mut map_parent = commands.spawn(SpatialBundle::default());

	let world = create_world(LEVEL_SIZE);
	add_sprites(&mut map_parent, &world);
}

fn create_world(width: usize) -> Vec<usize> {
	let mut heights: Vec<usize> = Vec::with_capacity(width);
	let mut height = 1;
	(0..width).for_each(|x| {
		heights.push(height);
		height = get_height(x)
	});
	heights
}

fn get_height(x: usize) -> usize {
	1 + ((x / 5) % 3)
}

fn add_sprites(commands: &mut EntityCommands, world: &Vec<usize>) {
	world.iter().enumerate().for_each(|(x, height)| {
		add_tile(commands, x as f32, *height);
	});
}

fn add_tile(commands: &mut EntityCommands, x: f32, height: usize) {
	commands.with_children(|parent| {
		parent.spawn(SpriteBundle {
			sprite: Sprite {
				custom_size: Some(Vec2::new(1., height as f32)),
				color: Color::rgb(0.9, 0.9, 0.9),
				..Default::default()
			},
			transform: Transform::from_xyz(x, height as f32 / 2., 0.),
			..Default::default()
		});
	});
}
