use bevy::prelude::*;
use serde::Deserialize;

use super::BindId;

//TODO: move it elsewhere
#[derive(Debug, Deserialize)]
pub enum TransformMode {
	Absolute,
	Relative,
}

#[derive(Debug, Deserialize)]
pub struct Action {
	pub(crate) bind_id: BindId,
	pub(crate) transform: Vec2, // HACK: data type should be dynamic and related to project
	pub(crate) transform_mode: TransformMode,
}

impl Action {
	fn new(bind_id: BindId) -> Self {
		Self {
			bind_id,
			transform: Vec2::new(0., 0.),
			transform_mode: TransformMode::Absolute,
		}
	}
}
