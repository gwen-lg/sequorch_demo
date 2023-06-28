use serde::Deserialize;

use super::{event::Event, flow, FlowScale};

#[derive(Debug, Deserialize)]
pub struct Scene {
	flow_scale: FlowScale,
	events: Vec<Event>,
}

impl Default for Scene {
	fn default() -> Self {
		Self {
			flow_scale: flow::FLOW_TIME_MS,
			events: vec![],
		}
	}
}
