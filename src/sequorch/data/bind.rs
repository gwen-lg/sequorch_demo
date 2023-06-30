use serde::Deserialize;

#[derive(Default, Debug, Clone, Deserialize)] //TODO: Copy
pub struct BindId(String); //TODO: use unique name feature and numerical value comparison

impl BindId {
	pub fn from(name: impl ToString) -> Self {
		Self(name.to_string())
	}
}

#[derive(Default, Debug, Deserialize)] //Not resource, but asset (Add serilize ?)
pub struct Bind {
	id: BindId,
}
