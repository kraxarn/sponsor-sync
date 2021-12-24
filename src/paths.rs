use std::env::temp_dir;
use std::path::PathBuf;

pub fn cache() -> PathBuf {
	let mut path = temp_dir();
	path.push(crate::consts::FILE_NAME);
	path
}