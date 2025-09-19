macro_rules! is_media_extension {
	() => {
		Some("mp4" | "mkv")
	};
}
pub(super) use is_media_extension;

macro_rules! is_image_extension {
	() => {
		Some("png" | "jpg")
	};
}
pub(super) use is_image_extension;
