use bevy::prelude::*;

use crate::sequorch::SequOrchData;

use super::entity_bind::EntityBinding;

#[derive(Default, Debug)]
pub struct Progress(pub f32);

#[derive(Component, Debug)]
pub struct SceneInst {
	pub(crate) asset: Handle<SequOrchData>,
	pub(crate) entities_binding: Vec<EntityBinding>,
	pub(crate) progress: Progress,
}
