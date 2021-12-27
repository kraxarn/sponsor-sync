use std::collections::HashSet;
use std::env::var;

use clap::ArgMatches;
use sqlx::{Pool, Postgres, Row};
use sqlx::postgres::PgPoolOptions;

use crate::sponsor_time::SponsorTime;

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

	pub async fn up(&self) -> Result<(), sqlx::Error> {
		if let Err(e) = sqlx::query(include_str!("../sql/sponsor_time/up.sql"))
			.execute(&self.pool).await {
			return Err(e);
		}

		if let Err(e) = sqlx::query(include_str!("../sql/sponsor_time_video_id_idx/up.sql"))
			.execute(&self.pool).await {
			return Err(e);
		}

		Ok(())
	}

	pub async fn down(&self) -> Result<(), sqlx::Error> {
		if let Err(e) = sqlx::query(include_str!("../sql/sponsor_time_video_id_idx/down.sql"))
			.execute(&self.pool).await {
			return Err(e);
		}

		if let Err(e) = sqlx::query(include_str!("../sql/sponsor_time/down.sql"))
			.execute(&self.pool).await {
			return Err(e);
		}

		Ok(())
	}

	pub async fn exists(&self, id: &str) -> bool {
		sqlx::query("select 1 from sponsor_time where id = $1")
			.bind(id)
			.fetch_optional(&self.pool)
			.await.unwrap()
			.is_some()
	}

	pub async fn get_ids(&self) -> HashSet<String> {
		let ids = sqlx::query("select id from sponsor_time")
			.fetch_all(&self.pool)
			.await.unwrap();

		let mut set = HashSet::new();
		for id in ids {
			set.insert(id.get(0));
		}
		set
	}

	/// Add a sponsor time to the database.
	/// Returns rows affected on success, otherwise, error.
	pub async fn add(&self, time: &SponsorTime) -> Result<u64, sqlx::Error> {
		let result = sqlx::query("insert into sponsor_time
				(id, video_id, start_time, end_time)
			values ($1, $2, $3, $4)")
			.bind(&time.id)
			.bind(&time.get_video_id())
			.bind(time.start_time)
			.bind(time.end_time)
			.execute(&self.pool)
			.await;

		match result {
			Ok(q) => Ok(q.rows_affected()),
			Err(e) => Err(e),
		}
	}
}