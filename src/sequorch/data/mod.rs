mod group;

use bevy::reflect::TypeUuid;
use serde::Deserialize;

pub use group::Group;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct SequOrchData {
	pub groups: Vec<Group>,
}
