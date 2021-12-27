use std::io::ErrorKind;

pub struct Indexes {
	/// Index of UUID
	pub id: usize,
	/// Index of video ID
	pub video_id: usize,
	/// Index of start time
	pub start_time: usize,
	/// Index of end time
	pub end_time: usize,
}

impl Indexes {
	fn empty() -> Self {
		Self {
			id: usize::MAX,
			video_id: usize::MAX,
			start_time: usize::MAX,
			end_time: usize::MAX,
		}
	}

	pub fn new(line: &str) -> Result<Self, std::io::Error> {
		let mut indexes = Self::empty();
		let parts: Vec<&str> = line.split(crate::consts::CSV_SEPARATOR).collect();

		for i in 0..parts.len() {
			match parts[i] {
				"videoID" => indexes.video_id = i,
				"startTime" => indexes.start_time = i,
				"endTime" => indexes.end_time = i,
				"UUID" => indexes.id = i,
				_ => {}
			}
		}

		match indexes.get_all_invalid() {
			Some(i) => Err(std::io::Error::new(
				ErrorKind::InvalidData,
				format!("invalid fields: {}", i.join(", ")))),
			None => Ok(indexes),
		}
	}

	fn get_all_invalid(&self) -> Option<Vec<&str>> {
		let empty = Self::empty();
		let mut invalid = Vec::new();

		if self.id == empty.id {
			invalid.push("id");
		}

		if self.video_id == empty.video_id {
			invalid.push("video_id");
		}

		if self.start_time == empty.start_time {
			invalid.push("start_time");
		}

		if self.end_time == empty.end_time {
			invalid.push("end_time");
		}

		if invalid.is_empty() {
			None
		} else {
			Some(invalid)
		}
	}
}