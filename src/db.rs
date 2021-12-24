use std::env::var;
use clap::ArgMatches;
use sqlx::{Pool, Postgres, query};
use sqlx::postgres::{PgPoolOptions, PgQueryResult};

pub struct Db {
	pool: Pool<Postgres>,
}

impl Db {
	pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
		Ok(Self {
			pool: PgPoolOptions::new()
				.connect(&database_url)
				.await?,
		})
	}

	pub fn get_url(matches: &ArgMatches) -> String {
		match matches.value_of(crate::args::DATABASE_URL) {
			Some(s) => s.to_owned(),
			None => match var("DATABASE_URL") {
				Ok(v) => v,
				Err(_) => panic!("database url not specified"),
			}
		}
	}

	pub async fn up(&self) -> Result<PgQueryResult, sqlx::Error> {
		query(include_str!("../sql/up.sql"))
			.execute(&self.pool)
			.await
	}
}