use bevy::{prelude::*, window::PresentMode};

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
		.run();
}
