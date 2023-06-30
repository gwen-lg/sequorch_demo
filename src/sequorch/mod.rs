pub mod data;
pub mod run;

pub use self::data::SequOrchData;

use bevy::prelude::*;

use self::{
	data::TransformMode,
	run::{SceneInst, SequOrchTransform, TeleportEvent},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SequOrchSet {
	//ProcessInputData,
	Update,
}
pub struct SequOrchPlugin;

impl Plugin for SequOrchPlugin {
	fn build(&self, app: &mut App) {
		app.add_asset::<SequOrchData>()
			.add_event::<TeleportEvent>()
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
	mut ev_teleport: EventWriter<TeleportEvent>,
	mut scenes: Query<&mut SceneInst>,
) {
	let mut teleport_events = Vec::new();

	let delta_time = time.delta();
	scenes.for_each_mut(|mut scene_inst| {
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

		let mut events = actions
			.flat_map(|action| {
				let entities = scene_inst.get_entities(action.bind_id.clone());
				entities.into_iter().map(|entity| TeleportEvent {
					entity,
					transform: action.transform,
					transform_mode: action.transform_mode,
				})
				//println!("[{new_progress:?}]: start action '{action:?}'");
			})
			.collect();

		teleport_events.append(&mut events);
	});

	teleport_events.into_iter().for_each(|teleport_event| {
		ev_teleport.send(teleport_event);
	});
}

pub fn update_transform(
	mut ev_teleport: EventReader<TeleportEvent>,
	mut entities: Query<&mut Transform, With<SequOrchTransform>>,
) {
	ev_teleport.iter().for_each(|teleport| {
		let mut transform = entities.get_mut(teleport.entity).unwrap();
		match teleport.transform_mode {
			TransformMode::Absolute => {
				transform.translation.x = teleport.transform.x;
				transform.translation.y = teleport.transform.y;
			}
			TransformMode::Relative => {
				transform.translation.x += teleport.transform.x;
				transform.translation.y += teleport.transform.y;
			}
		}
	});
}
