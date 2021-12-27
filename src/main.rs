use std::fs::remove_file;
use std::path::PathBuf;

use env_logger::Env;
use log::{debug, info, warn};

use crate::db::Db;
use crate::sponsor_times::SponsorTimes;

mod app;
mod args;
mod consts;
mod db;
mod http;
mod indexes;
mod paths;
mod sponsor_time;
mod sponsor_times;

/// Log added entry
fn add_current(current: &mut usize, total: usize) {
	*current += 1;
	if *current % 100_000_usize == 0 {
		debug!("{:>8}/{:<8} ({:>3.0}%)", current, total,
				*current as f32 / total as f32 * 100_f32);
	}
}

#[tokio::main]
async fn main() {
	// By default, log everything from current crate
	env_logger::Builder::from_env(Env::default()
		.default_filter_or("sponsor_sync"))
		.init();

	let default_cache = paths::cache();
	let app = app::new(&default_cache);

	let matches = app.get_matches();

	let cache_path = match matches.value_of(args::CACHE_DIR) {
		Some(c) => PathBuf::from(c),
		None => paths::cache(),
	};

	info!("Connecting to database");

	let database_url = Db::get_url(&matches);
	let db = Db::connect(&database_url).await.unwrap();

	if matches.is_present(args::RESET_DATABASE) {
		info!("Resetting database");
		db.down().await.unwrap();
	}

	info!("Updating database");

	db.up().await.unwrap();

	if matches.is_present(args::USE_CACHE) && cache_path.exists() {
		info!("Using cached file");
	} else {
		info!("Downloading to: {:?}", &cache_path);
		let mirror = matches.value_of(args::MIRROR).unwrap();
		http::download(mirror, &cache_path).await;
	}

	info!("Adding data to database");

	let mut ignored = Vec::new();

	let mut times = SponsorTimes::new(&cache_path).unwrap();
	let mut current = 0_usize;
	let total = times.total_entries();

	let existing = if matches.is_present(args::LOW_MEMORY) {
		Some(db.get_ids().await)
	} else {
		None
	};

	for time in &mut times {
		if time.start_time == 0_f32 && time.end_time == 0_f32 {
			ignored.push((time, "invalid interval".to_owned()));
			continue;
		}

		if match &existing {
			Some(s) => s.contains(&time.id),
			None => db.exists(&time.id).await
		} {
			add_current(&mut current, total);
			continue;
		}

		if let Err(e) = db.add(&time).await {
			ignored.push((time, match e.as_database_error() {
				Some(e) => e.message(),
				None => "database error",
			}.to_owned()));
		}

		add_current(&mut current, total);
	}

	if !matches.is_present(args::USE_CACHE) {
		remove_file(&cache_path).unwrap();
	}

	if ignored.len() > 0 {
		let len = ignored.len();
		warn!("{} {} were ignored", len, if len == 1 {
			"item"
		} else {
			"items"
		});
	}
}
