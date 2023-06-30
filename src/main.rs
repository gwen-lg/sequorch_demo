mod app;
mod game;
mod sequorch;

use crate::game::GamePlugin;

use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_editor_pls::prelude::*;
use clap::Parser;
use sequorch::SequOrchPlugin;

fn main() {
	let args = app::Args::parse();

	let mut bevy_app = App::new();
	bevy_app
		.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "SequOrch Demo".to_string(),
						resolution: (1280.0, 768.0).into(),
						present_mode: PresentMode::Immediate,
						..Default::default()
					}),
					..Default::default()
				})
				.build()
				.disable::<AudioPlugin>(),
		)
		.add_plugin(GamePlugin)
		.add_plugin(SequOrchPlugin);

	if args.editor_pls() {
		bevy_app.add_plugin(EditorPlugin::default());
	}

	bevy_app.run();
}
