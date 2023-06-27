mod door;
mod level;
mod npc;

use std::time::Duration;

use crate::sequorch::{self, SequOrchData};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use door::Door;

#[derive(Resource)]
pub struct HardcodedAssets {
	door_gr: Handle<SequOrchData>,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup_camera)
			.add_startup_system(level::setup_test_map)
			.add_startup_system(setup_doors)
			.add_startup_system(setup_npcs)
			.add_startup_system(load_sequorch_assets)
			.add_system(start_sequorch_on_door.run_if(on_timer(Duration::from_secs_f32(5.))));
	}
}

const VIEWPORT_CAM_VIEW: Vec2 = Vec2::new(1280., 768.);

pub fn setup_camera(mut commands: Commands) {
	let mut camera = Camera2dBundle::default();
	camera.projection = OrthographicProjection {
		far: 1000.0,
		scale: 0.1,
		..Default::default()
	};
	camera.transform.translation =
		Vec3::new(VIEWPORT_CAM_VIEW.x / 20., VIEWPORT_CAM_VIEW.y / 20., 999.0);

	commands.spawn(camera);
}

pub fn setup_doors(mut commands: Commands) {
	const DOOR_SIZE: Vec2 = Vec2::new(10., 10.);
	door::spawn_door(&mut commands, DOOR_SIZE, Transform::from_xyz(10., 1.0, 0.));
	door::spawn_door(&mut commands, DOOR_SIZE, Transform::from_xyz(50., 1.0, 0.));
	door::spawn_door(&mut commands, DOOR_SIZE, Transform::from_xyz(105., 1.0, 0.));
}

pub fn setup_npcs(mut commands: Commands) {
	npc::spawn_npc(&mut commands, Transform::from_xyz(5., 2., 3.));
	npc::spawn_npc(&mut commands, Transform::from_xyz(35., 2., 3.));
	npc::spawn_npc(&mut commands, Transform::from_xyz(65., 2., 3.));
}

pub fn load_sequorch_assets(
	mut commands: Commands,
	mut sequorch_data: ResMut<Assets<SequOrchData>>,
	//hardcoded_assets: ResMut<HardcodedAssets>,
) {
	let test_sequorch_data = SequOrchData { groups: vec![] }; //TODO
	let handle = sequorch_data.add(test_sequorch_data);
	commands.insert_resource(HardcodedAssets { door_gr: handle });
}

pub fn start_sequorch_on_door(
	mut commands: Commands,
	//group: Res<sequorch::data::Group>,
	doors: Query<Entity, &Door>,
	hardcoded_assets: Res<HardcodedAssets>,
) {
	println!("start_sequorch_on_door");
	let doors = doors.iter().collect::<Vec<_>>();
	let rand_door = doors[0]; //TODO: get random door
	commands.spawn(sequorch::run::GroupInst {
		entity: rand_door,
		asset: hardcoded_assets.door_gr.clone(),
	});
}
