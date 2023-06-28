use serde::Deserialize;

use super::{Action, FlowProgress};

#[derive(Debug, Deserialize)]
pub struct Event {
	flow_position: FlowProgress,
	actions: Vec<Action>,
}
