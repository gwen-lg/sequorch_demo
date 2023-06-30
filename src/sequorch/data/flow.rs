use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct TimeUnit(u32);

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct SubUnit(u32);

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum FlowScale {
	Time(TimeUnit),
	Subdivision(SubUnit),
}

// one unit correspond to one centisecond
pub const FLOW_TIME_CS: FlowScale = FlowScale::Time(TimeUnit(100));
pub const FLOW_PERCENT: FlowScale = FlowScale::Subdivision(SubUnit(100));
pub const FLOW_PERTHOUSAND: FlowScale = FlowScale::Subdivision(SubUnit(1000));

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Clone, Copy)]
pub struct FlowProgress(u32);

impl FlowProgress {
	pub fn new(progress: u32) -> Self {
		Self(progress)
	}

	/// TODO: improve compute management
	pub fn from(flow_scale: FlowScale, progress: f32) -> Self {
		match flow_scale {
			FlowScale::Time(time) => {
				let TimeUnit(time) = time;
				let progress = progress * time as f32;
				FlowProgress::new(progress as u32)
			}
			FlowScale::Subdivision(sub) => {
				let SubUnit(sub) = sub;
				let progress = progress / sub as f32;
				FlowProgress::new(progress as u32)
			}
		}
	}
}
