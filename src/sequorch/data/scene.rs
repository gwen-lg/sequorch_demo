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
			flow_scale: flow::FLOW_TIME_CS,
			events: vec![],
		}
	}
}

impl Scene {
	pub fn new(flow_scale: FlowScale) -> Self {
		Self {
			flow_scale,
			events: vec![],
		}
	}

	pub fn flow_scale(&self) -> FlowScale {
		self.flow_scale
	}

	pub fn events(&self) -> &Vec<Event> {
		&self.events
	}

	pub fn add_events(&mut self, events: &mut Vec<Event>) {
		self.events.append(events);
		//TODO: sort events
	}
}
