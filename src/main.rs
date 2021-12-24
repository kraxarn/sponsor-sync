use std::path::PathBuf;

use env_logger::Env;
use log::info;
use crate::db::Db;

mod app;
mod args;
mod consts;
mod db;
mod http;
mod paths;

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

	info!("Updating database");

	db.up().await.unwrap();

	info!("Downloading to: {:?}", &cache_path);

	let mirror = matches.value_of(args::MIRROR).unwrap();
	http::download(mirror, &cache_path).await;

	Ok(())
}
