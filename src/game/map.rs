use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

const LEVEL_SIZE: usize = 150;

pub fn spawn_floor(mut commands: Commands) {
	let mut map_parent = commands.spawn(SpatialBundle::default());

	let world = create_world(LEVEL_SIZE);
	add_sprites(&mut map_parent, &world);
}

fn create_world(width: usize) -> Vec<usize> {
	let mut heights: Vec<usize> = Vec::with_capacity(width);
	let mut height = 1;
	(0..width).for_each(|_| {
		heights.push(height);
		height = get_next_height(height)
	});
	heights
}

fn get_next_height(current_height: usize) -> usize {
	const MAX_MAP_HEIGHT: usize = 5;
	let next_height = current_height as isize + get_random_height_delta();
	return if next_height > 0 {
		std::cmp::min(next_height as usize, MAX_MAP_HEIGHT)
	} else {
		1
	};
}
fn get_random_height_delta() -> isize {
	let mut rng = thread_rng();
	let random_number: u32 = rng.gen_range(0..100);
	let delta = match random_number {
		0..=70 => 0,
		71..=82 => -1,
		83..=94 => 1,
		95..=96 => 2,
		97..=98 => -2,
		_ => 0,
	};
	delta
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
