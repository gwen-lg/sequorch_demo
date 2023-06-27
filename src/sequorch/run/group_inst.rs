use bevy::prelude::*;

use crate::sequorch::data::SequOrchData;

#[derive(Component, Debug)]
pub struct GroupInst {
	pub entity: Entity,
	// add associate group resource
	pub asset: Handle<SequOrchData>,
}
