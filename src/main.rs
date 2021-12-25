use std::path::PathBuf;

use env_logger::Env;
use log::{debug, error, info};

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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
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

	for time in SponsorTimes::new(&cache_path).unwrap() {
		if time.start_time == 0_f32 && time.end_time == 0_f32 {
			continue;
		}

		if db.exists(&time.id).await {
			continue;
		}

		if let Err(e) = db.add(&time).await {
			error!("{:?}", e);
		}

		debug!("Added {}", time.id);
	}

	Ok(())
}
