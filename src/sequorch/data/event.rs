use serde::Deserialize;

use super::{Action, FlowProgress};

#[derive(Debug, Deserialize)]
pub struct Event {
	pub(crate) flow_position: FlowProgress,
	pub(crate) actions: Vec<Action>,
}

impl Event {
	pub fn position(&self) -> FlowProgress {
		self.flow_position
	}

	pub fn actions(&self) -> &Vec<Action> {
		&self.actions
	}
}
