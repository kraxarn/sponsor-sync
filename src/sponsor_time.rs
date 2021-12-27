use std::borrow::Borrow;
use url::Url;
use crate::indexes::Indexes;

pub struct SponsorTime {
	pub id: String,
	pub video_id: String,
	pub start_time: f32,
	pub end_time: f32,
}

impl SponsorTime {
	pub fn parse(parts: &Vec<&str>, i: &Indexes) -> Option<Self> {
		match (parts[i.start_time].parse(), parts[i.end_time].parse()) {
			(Ok(s), Ok(e)) => Some(Self {
				id: parts[i.id].to_owned(),
				video_id: parts[i.video_id].to_owned(),
				start_time: s,
				end_time: e,
			}),
			_ => None,
		}
	}

	/// Parse video ID as URL, or return as-is
	pub fn get_video_id(&self) -> String {
		if !self.video_id.starts_with("https://") {
			return self.video_id.to_owned();
		}

		if let Ok(u) = Url::parse(&self.video_id) {
			if let Some(p) = u.query_pairs().find(|pair| {
				pair.0 == "v"
			}) {
				let value: &str = p.1.borrow();
				return value.to_owned();
			}
		}

		self.video_id.to_owned()
	}
}