use serde::Deserialize;

use super::{Action, FlowProgress};

#[derive(Debug, Deserialize)]
pub struct Event {
	pub(crate) flow_position: FlowProgress,
	pub(crate) actions: Vec<Action>,
}
