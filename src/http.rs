use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use reqwest::Url;
use futures_util::StreamExt;

pub async fn download(mirror: &str, destination: &PathBuf) {
	let url = mirror
		.parse::<Url>().unwrap()
		.join(crate::consts::FILE_NAME).unwrap();

	let mut out = File::create(destination).unwrap();
	let response = reqwest::get(url).await.unwrap();

	let mut stream = response.bytes_stream();
	while let Some(s) = stream.next().await {
		out.write(&s.unwrap()).unwrap();
	}
}