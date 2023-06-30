use serde::Deserialize;

#[derive(Default, Debug, Clone, Deserialize, PartialEq)] //TODO: Copy
pub struct BindId(String); //TODO: use unique name feature and numerical value comparison

impl BindId {
	pub fn from(name: impl ToString) -> Self {
		Self(name.to_string())
	}
}
