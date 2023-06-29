use std::{
	ops::{Add, AddAssign},
	time::Duration,
};

use bevy::prelude::*;

use crate::sequorch::SequOrchData;

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

#[derive(Component, Debug)]
pub struct SceneInst {
	pub(crate) asset: Handle<SequOrchData>,
	pub(crate) entities_binding: Vec<EntityBinding>,
	pub(crate) progress: Progress,
}

impl SceneInst {
	pub(crate) fn update_time(&mut self, delta: Duration) {
		self.progress += delta;
	}

	pub fn asset(&self) -> &Handle<SequOrchData> {
		&self.asset
	}

	pub fn progress(&self) -> Progress {
		self.progress
	}
}
