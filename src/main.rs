mod app;
mod game;
mod sequorch;

use crate::game::GamePlugin;

use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use clap::Parser;
use sequorch::SequOrchPlugin;

#[cfg(feature = "editor")]
use bevy_editor_pls::prelude::*;

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
		.add_plugins(GamePlugin)
		.add_plugins(SequOrchPlugin);

	if args.editor_pls() {
		#[cfg(feature = "editor")]
		bevy_app.add_plugins(EditorPlugin::default());

		#[cfg(not(feature = "editor"))]
		println!("can't enbale editor, feature is not enabled");
	}

	bevy_app.run();
}
