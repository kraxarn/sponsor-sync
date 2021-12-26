use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use log::debug;

use crate::indexes::Indexes;
use crate::sponsor_time::SponsorTime;

pub struct SponsorTimes {
	total: usize,
	reader: BufReader<File>,
	indexes: Indexes,
}

impl SponsorTimes {
	pub fn new(path: &PathBuf) -> std::io::Result<Self> {
		let file = File::open(path)?;
		let mut reader = BufReader::new(file);

		let mut buffer = String::new();
		reader.read_line(&mut buffer)?;

		let indexes = match Indexes::new(&buffer) {
			Ok(i) => i,
			Err(e) => panic!("missing fields: {}", e),
		};

		Ok(Self {
			total: Self::get_total_entries(path)?,
			reader,
			indexes,
		})
	}

	fn get_total_entries(path: &PathBuf) -> std::io::Result<usize> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		Ok(reader.lines().count())
	}

	pub fn total_entries(&self) -> usize {
		self.total
	}
}

impl Iterator for SponsorTimes {
	type Item = SponsorTime;

	fn next(&mut self) -> Option<Self::Item> {
		let mut buffer = String::new();
		let bytes = self.reader.read_line(&mut buffer).unwrap();

		if bytes == usize::MIN {
			return None;
		}

		let i = &self.indexes;
		let max = max(i.id,
					  max(i.video_id,
						  max(i.start_time, i.end_time)));

		let parts: Vec<&str> = buffer.split(crate::consts::CSV_SEPARATOR).collect();
		if parts.len() < max {
			debug!("ignoring line with missing values: {:?}", buffer);
			return self.next();
		}

		Some(Self::Item {
			id: parts[i.id].to_owned(),
			video_id: parts[i.video_id].to_owned(),
			start_time: parts[i.start_time].parse().unwrap(),
			end_time: parts[i.end_time].parse().unwrap(),
		})
	}
}