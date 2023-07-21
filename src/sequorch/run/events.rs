use crate::sequorch::data::TransformMode;
use bevy::prelude::*;

#[derive(Clone, Event)]
pub struct TeleportEvent {
	pub entity: Entity,
	pub transform: Vec2,
	pub transform_mode: TransformMode,
}
