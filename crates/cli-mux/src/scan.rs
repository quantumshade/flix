use std::path::Path;

use walkdir::WalkDir;

use crate::model::MediaFile;
use crate::probe::probe_file;

pub fn scan_directory<T>(
	path: &Path,
	unknown_length_start: impl FnOnce() -> T,
	unknown_length_end: impl FnOnce(T),
	fixed_length_start: impl FnOnce(usize) -> T,
	fixed_length_update: impl Fn(&mut T),
	fixed_length_end: impl FnOnce(T),
	print_fn: impl Fn(&T, &str),
) -> (Vec<MediaFile>, u64) {
	let spinner = unknown_length_start();
	let files: Vec<_> = WalkDir::new(path)
		.into_iter()
		.filter_map(Result::ok)
		.filter(|e| e.file_type().is_file())
		.filter(|e| {
			let is_mkv = e
				.path()
				.extension()
				.unwrap_or_default()
				.eq_ignore_ascii_case("mkv");
			let is_mp4 = e
				.path()
				.extension()
				.unwrap_or_default()
				.eq_ignore_ascii_case("mp4");
			is_mkv || is_mp4
		})
		.collect();
	unknown_length_end(spinner);

	let mut progress = fixed_length_start(files.len());
	let mut total_byte_size = 0u64;
	let files: Vec<_> = files
		.iter()
		.filter_map(|entry| {
			let file = match probe_file(entry) {
				Ok(file) => Some(file),
				Err(err) => {
					print_fn(&progress, &format!("{:?}", err));
					None
				}
			};
			let size = file.as_ref().map(|r| r.byte_size).unwrap_or_default();
			total_byte_size = total_byte_size.saturating_add(size);
			fixed_length_update(&mut progress);
			file
		})
		.collect();
	fixed_length_end(progress);
	(files, total_byte_size)
}
