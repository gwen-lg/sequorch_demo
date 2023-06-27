use bevy::prelude::*;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)] //Not resource, but asset (Add serilize ?)
pub struct Group {
	pub keys_tmp: Vec<Vec2>,
}
