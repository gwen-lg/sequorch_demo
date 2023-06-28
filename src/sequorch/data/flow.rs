use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TimeUnit(u32);

#[derive(Debug, Deserialize)]
pub struct SubUnit(u32);

#[derive(Debug, Deserialize)]
pub enum FlowScale {
	Time(TimeUnit),
	Subdivision(SubUnit),
}

pub const FLOW_TIME_MS: FlowScale = FlowScale::Time(TimeUnit(100));
pub const FLOW_PERCENT: FlowScale = FlowScale::Subdivision(SubUnit(100));
pub const FLOW_PERTHOUSAND: FlowScale = FlowScale::Subdivision(SubUnit(1000));

#[derive(Debug, Deserialize)]
pub struct FlowProgress(u32);
