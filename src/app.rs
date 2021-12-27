use std::env::var;
use std::path::PathBuf;
use clap::{App, Arg};

pub fn new(default_cache: &PathBuf) -> App {
	let mirror = Arg::with_name(crate::args::MIRROR)
		.short("m")
		.long("mirror")
		.value_name("URL")
		.help("Mirror to use, more info: https://github.com/mchangrh/sb-mirror")
		.takes_value(true)
		.required(true);

	let database = Arg::with_name(crate::args::DATABASE)
		.short("d")
		.long("database")
		.value_name("NAME")
		.help("Database to use")
		.possible_values(&["postgres", "mysql", "sqlite"])
		.takes_value(true)
		.required(true);

	let cache_dir = Arg::with_name(crate::args::CACHE_DIR)
		.short("c")
		.long("cache-dir")
		.value_name("PATH")
		.help("Directory to store temporary files")
		.takes_value(true)
		.default_value(default_cache.to_str().unwrap());

	let database_url = Arg::with_name(crate::args::DATABASE_URL)
		.short("u")
		.long("database-url")
		.value_name("URL")
		.help("URL to database, can also be specified with DATABASE_URL env")
		.takes_value(true)
		.required(var("DATABASE_URL").is_err());

	let reset_database = Arg::with_name(crate::args::RESET_DATABASE)
		.short("r")
		.long("reset")
		.help("Reset all current values in the database first");

	let use_cache = Arg::with_name(crate::args::USE_CACHE)
		.short("k")
		.long("use-cache")
		.help("Use cached file, if it exists, instead of downloading a new one");

	let low_memory = Arg::with_name(crate::args::LOW_MEMORY)
		.short("m")
		.long("low-memory")
		.help("Minimize memory usage, at the cost of longer execution time");

	App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.arg(mirror)
		.arg(database)
		.arg(cache_dir)
		.arg(database_url)
		.arg(reset_database)
		.arg(use_cache)
		.arg(low_memory)
}