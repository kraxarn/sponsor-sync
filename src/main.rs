use std::env::var;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches};
use env_logger::Env;
use futures_util::StreamExt;
use log::info;
use reqwest::Url;
use sqlx::postgres::PgPoolOptions;
use sqlx::query;

mod arg;

static FILE_NAME: &str = "sponsorTimes.csv";

fn cache_path() -> PathBuf {
	let mut path = std::env::temp_dir();
	path.push(FILE_NAME);
	path
}

fn get_database_url(matches: &ArgMatches) -> String {
	match matches.value_of("database-url") {
		Some(s) => s.to_owned(),
		None => match var("DATABASE_URL") {
			Ok(v) => v,
			Err(_) => panic!("database url not specified"),
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	// By default, log everything from current crate
	env_logger::Builder::from_env(Env::default()
		.default_filter_or("sponsor_sync"))
		.init();

	let default_cache = cache_path();

	let app = App::new("sponsor-sync")
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.arg(Arg::with_name("mirror")
			.short("m")
			.long("mirror")
			.value_name("URL")
			.help("Mirror to use, more info: https://github.com/mchangrh/sb-mirror")
			.takes_value(true)
			.required(true))
		.arg(Arg::with_name("cache-dir")
			.short("c")
			.long("cache-dir")
			.value_name("PATH")
			.help("Directory to store temporary files")
			.takes_value(true)
			.default_value(default_cache.to_str().unwrap()))
		.arg(Arg::with_name("database")
			.short("d")
			.long("database")
			.value_name("NAME")
			.help("Database to use")
			.possible_values(&["postgres", "mysql", "sqlite"])
			.takes_value(true)
			.required(true))
		.arg(Arg::with_name("database-url")
			.short("u")
			.long("database-url")
			.value_name("URL")
			.help("URL to database, can also be specified with DATABASE_URL env")
			.takes_value(true)
			.required(var("DATABASE_URL").is_err()));

	let matches = app.get_matches();
	let database_url = get_database_url(&matches);

	let cache_path = match matches.value_of("cache-dir") {
		Some(c) => PathBuf::from(c),
		None => cache_path(),
	};

	info!("Connecting to database");

	let pool = PgPoolOptions::new()
		.connect(&database_url)
		.await?;

	info!("Updating database");

	query(include_str!("../sql/up.sql"))
		.execute(&pool)
		.await?;

	let url = matches
		.value_of("mirror").unwrap()
		.parse::<Url>().unwrap()
		.join(FILE_NAME)
		.unwrap();

	info!("Downloading {} to: {:?}", url, &cache_path);

	let mut out = File::create(&cache_path).unwrap();
	let response = reqwest::get(url).await.unwrap();
	let mut stream = response.bytes_stream();

	while let Some(s) = stream.next().await {
		out.write(&s.unwrap()).unwrap();
	}

	Ok(())
}
