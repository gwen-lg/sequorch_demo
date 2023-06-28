use serde::Deserialize;

#[derive(Default, Debug, Deserialize)] //Not resource, but asset (Add serilize ?)
pub struct Bind {
	id: String, //TODO: use unique name feature
}
