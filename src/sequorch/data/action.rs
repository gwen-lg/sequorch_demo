use bevy::prelude::*;
use serde::Deserialize; // Hack

#[derive(Debug, Deserialize)]
pub struct Action {
	new_pos: Vec2, // HACK: data type should be dynamic and related to project
}

impl Action {
	fn new() -> Self {
		Self {
			new_pos: Vec2::new(0., 0.),
		}
	}
}
