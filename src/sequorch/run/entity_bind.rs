use bevy::prelude::*;

use crate::sequorch::data::BindId;

#[derive(Debug)]
pub struct EntityBinding {
	pub(crate) entity: Entity,
	pub(crate) bind_id: BindId,
}
