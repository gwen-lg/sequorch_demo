mod action;
mod bind;
mod event;
mod flow;
mod scene;

use bevy::prelude::Vec2;
use bevy::reflect::TypeUuid;
use serde::Deserialize;

pub use action::{Action, TransformMode};
pub use bind::BindId;
pub use event::Event;
pub use flow::FlowProgress;
pub use flow::FlowScale;
pub use flow::FLOW_PERCENT;
pub use flow::FLOW_PERTHOUSAND;
pub use flow::FLOW_TIME_CS;
pub use scene::Scene;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct SequOrchData {
	pub scenes: Vec<Scene>,
}

pub fn create_test_sequorch_data() -> SequOrchData {
	let ent_bind = BindId::from("ent1");
	let mut scene = Scene::new(FLOW_TIME_CS);
	scene.add_events(&mut vec![
		Event {
			flow_position: FlowProgress::new(50),
			actions: vec![Action {
				bind_id: ent_bind.clone(),
				transform: Vec2::new(20.5, 10.),
				transform_mode: action::TransformMode::Absolute,
			}],
		}, // Action
		Event {
			flow_position: FlowProgress::new(150),
			actions: vec![Action {
				bind_id: ent_bind.clone(),
				transform: Vec2::new(20.5, 14.),
				transform_mode: action::TransformMode::Absolute,
			}],
		},
		Event {
			flow_position: FlowProgress::new(200),
			actions: vec![Action {
				bind_id: ent_bind,
				transform: Vec2::new(25.5, 18.),
				transform_mode: action::TransformMode::Absolute,
			}],
		},
	]);
	SequOrchData {
		scenes: vec![scene],
	}
}
