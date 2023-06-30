pub mod data;
pub mod run;

pub use self::data::SequOrchData;

use bevy::prelude::*;

use self::run::{SceneInst, SequOrchTransform};

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

pub fn update_scenes(
	time: Res<Time>,
	sequorchs: Res<Assets<SequOrchData>>,
	mut scenes: Query<&mut SceneInst>,
) {
	let delta_time = time.delta();
	scenes.par_iter_mut().for_each_mut(|mut scene_inst| {
		let sequorch_data = sequorchs.get(scene_inst.asset()).unwrap();

		let old_progress = scene_inst.flow_progress();
		scene_inst.update_time(delta_time);
		let new_progress = scene_inst.flow_progress();
		//println!("update_scenes : {delta_time:?} - {old_progress:?} -> {new_progress:?}");
		let scene = &sequorch_data.scenes[0];
		let scene_events = scene.events();

		let actions = scene_events
			.iter()
			.filter_map(|event| {
				let event_pos = event.position();
				if old_progress < event_pos && event_pos <= new_progress {
					Some(event.actions())
				} else {
					None
				}
			})
			.flatten(); // .collect::<Vec<_>>()

		actions.for_each(|action| {
			println!("[{new_progress:?}]: start action '{action:?}'");
		});
		//let sequorch_data = Assets::<SequOrchData>::get(self.asset);
	})
}
pub fn update_transform(
	_time: Res<Time>,
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
