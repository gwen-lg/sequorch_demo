use std::{
	ops::{Add, AddAssign},
	time::Duration,
};

use bevy::prelude::*;

use crate::sequorch::{
	data::{BindId, FlowProgress, FlowScale},
	SequOrchData,
};

use super::entity_bind::EntityBinding;

#[derive(Default, Debug, Clone, Copy)]
pub struct Progress(pub f32);

impl Add<Duration> for Progress {
	type Output = Self;

	fn add(self, duration: Duration) -> Self {
		let duration = duration.as_millis() as f32 / 1000.;
		Progress(self.0 + duration)
	}
}

impl AddAssign<Duration> for Progress {
	fn add_assign(&mut self, duration: Duration) {
		let duration = duration.as_millis() as f32 / 1000.;
		self.0 = self.0 + duration;
	}
}

impl From<f32> for Progress {
	fn from(value: f32) -> Self {
		Self(value)
	}
}

#[derive(Component, Debug)]
pub struct SceneInst {
	pub(crate) asset: Handle<SequOrchData>,
	pub(crate) entities_binding: Vec<EntityBinding>,
	pub(crate) progress: Progress,
	pub(crate) flow_progress: FlowProgress,
	pub(crate) flow_scale: FlowScale,
}

impl SceneInst {
	pub(crate) fn update_time(&mut self, delta: Duration) {
		self.progress += delta;

		let Progress(progress) = self.progress;
		self.flow_progress = FlowProgress::from(self.flow_scale, progress);
	}

	pub fn asset(&self) -> &Handle<SequOrchData> {
		&self.asset
	}

	pub fn flow_progress(&self) -> FlowProgress {
		self.flow_progress
	}

	pub fn get_entities(&self, bind_id: BindId) -> Vec<Entity> {
		self.entities_binding
			.iter()
			.filter_map(|ent_binding| {
				if ent_binding.bind_id == bind_id {
					Some(ent_binding.entity)
				} else {
					None
				}
			})
			.collect()
	}
}
