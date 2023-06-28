mod action;
mod bind;
mod event;
mod flow;
mod scene;

use bevy::reflect::TypeUuid;
use serde::Deserialize;

pub use action::Action;
pub use bind::Bind;
pub use flow::FlowProgress;
pub use flow::FlowScale;
pub use scene::Scene;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct SequOrchData {
	pub scenes: Vec<Scene>,
}
