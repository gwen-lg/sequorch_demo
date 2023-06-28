pub mod data;
pub mod run;

pub use self::data::SequOrchData;

use bevy::prelude::*;

use self::run::{GroupInst, SceneInst, SequOrchTransform};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SequOrchSet {
	//ProcessInputData,
	Update,
}
pub struct SequOrchPlugin;

impl Plugin for SequOrchPlugin {
	fn build(&self, app: &mut App) {
		app.add_asset::<SequOrchData>()
			.configure_sets(
				(
					CoreSet::StateTransitions,
					SequOrchSet::Update,
					CoreSet::FixedUpdate,
				)
					.chain(),
			)
			.add_system(update_scenes)
			.add_system(update_transform); // .in_set(SequOrchSet::Update)
	}
}

pub fn update_scenes(time: Res<Time>, mut scenes: Query<&mut SceneInst>) {
	scenes.par_iter().for_each(|scene| {
		println!("update scene : '{scene:?}'\n\t{time:#?}");
		// update time for groups ?
	})
}
pub fn update_transform(
	//time: Res<Time>,
	mut entities: Query<(Entity, &mut Transform), With<SequOrchTransform>>,
) {
	entities.par_iter().for_each(|(_entity, _transform)| {
		//let transform = entities.get(entity);
		//transform = transform

		//println!("update scene : '{transform:#?}'");
	});
}
/*
   for (mut transform, velocity) in &mut query {
	   transform.translation.x += velocity.x * time.delta_seconds();
	   transform.translation.y += velocity.y * time.delta_seconds();
   }
*/
