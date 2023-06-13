mod game;

use game::GamePlugin;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_editor_pls::prelude::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum WorldState {
	#[default]
	Run,
	Save,
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "SequOrch Demo".to_string(),
				resolution: (1280.0, 768.0).into(),
				present_mode: PresentMode::Immediate,
				..Default::default()
			}),
			..Default::default()
		}))
		.add_state::<WorldState>()
		//.add_system(save_world.in_shedule(OnEnter(WorldState::Save)))
		.add_system(exit_save_state.in_set(OnUpdate(WorldState::Save)))
		.add_plugin(EditorPlugin::default())
		.add_plugin(GamePlugin)
		.add_system(keyboard_input_system)
		.add_system(save_world)
		.run();
}

fn keyboard_input_system(
	keyboard_input: Res<Input<KeyCode>>,
	mut app_state: ResMut<NextState<WorldState>>,
) {
	if keyboard_input.pressed(KeyCode::S) {
		app_state.set(WorldState::Save);
		//TODO change state
	}
}

fn save_world(_world: &mut World) {
	// let mut query = world.query_filtered::<Entity, Without<NotInScene>>();
	// let entities: Vec<_> = query.iter(world).collect();

	// let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
	// let mut scene_builder =
	// 	DynamicSceneBuilder::from_world_with_type_registry(world, type_registry.clone());
	// scene_builder.extract_entities(entities.into_iter());
	// let scene = scene_builder.build();

	// let ron = scene.serialize_ron(type_registry);
	// match ron {
	// 	Ok(ron) => {
	// 		println!("Scene : {ron}");
	// 	}
	// 	Err(err) => {
	// 		println!("Error : {err}");
	// 	}
	// }
}

fn exit_save_state(mut app_state: ResMut<NextState<WorldState>>) {
	app_state.set(WorldState::Run);
}
