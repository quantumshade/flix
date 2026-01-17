//! This module contains helper functions for normalizing media titles

use core::iter::Peekable;

use itertools::Itertools;

/// # Panics
///
/// Panics if `input` is not ASCII.
fn split_normalized_words(input: &str) -> impl Iterator<Item = String> {
	if !input.is_ascii() {
		panic!("Input is not ASCII: {input}");
	}

	input.split_ascii_whitespace().map(|s| {
		let chars = s
			.chars()
			.filter(|c| c.is_ascii_alphanumeric() || *c == '-')
			.map(|c| c.to_ascii_lowercase());

		if s.len() > 4
			&& s.len().is_multiple_of(2)
			&& chars.clone().tuples().all(|(l, r)| l != '.' && r == '.')
		{
			// Collapse acronym
			chars.tuples().map(|(l, _)| l).collect()
		} else {
			chars.collect()
		}
	})
}

fn split_leading_article<I: Iterator<Item = String>>(iter: I) -> (Option<String>, Peekable<I>) {
	let mut iter = iter.peekable();
	match iter.peek().map(String::as_str) {
		Some("a" | "an" | "the") => (iter.next(), iter),
		_ => (None, iter),
	}
}

/// Convert a media title to be sortable and searchable
///
///     use flix_model::text::make_sortable_title;
///
///     assert_eq!(make_sortable_title("The Matrix"), "matrix, the");
///     assert_eq!(make_sortable_title("Marvel's Agents of S.H.I.E.L.D."), "marvels agents of shield");
///     assert_eq!(make_sortable_title("Avatar: The Last Airbender"), "avatar the last airbender");
///
/// # Panics
///
/// Panics if `input` is not ASCII.
pub fn make_sortable_title(title: &str) -> String {
	let words = split_normalized_words(title);
	let (article, words) = split_leading_article(words);

	let output = Itertools::intersperse(words, " ".to_string());
	if let Some(article) = article {
		output.chain([", ".to_string(), article]).collect()
	} else {
		output.collect()
	}
}

/// Convert a media title to a folder name representable on filesystems
///
///     use flix_model::text::make_fs_slug;
///
///     assert_eq!(make_fs_slug("The Matrix"), "matrix");
///     assert_eq!(make_fs_slug("Marvel's Agents of S.H.I.E.L.D."), "marvels agents of shield");
///     assert_eq!(make_fs_slug("Avatar: The Last Airbender"), "avatar the last airbender");
///
/// # Panics
///
/// Panics if `input` is not ASCII.
pub fn make_fs_slug(title: &str) -> String {
	let words = split_normalized_words(title);
	let (_, words) = split_leading_article(words);

	Itertools::intersperse(words, " ".to_string()).collect()
}

/// Convert a media title and year to a folder name representable on filesystems
///
///     use flix_model::text::make_fs_slug_year;
///
///     assert_eq!(make_fs_slug_year("The Matrix", 1999), "matrix (1999)");
///     assert_eq!(make_fs_slug_year("Marvel's Agents of S.H.I.E.L.D.", 2013), "marvels agents of shield (2013)");
///     assert_eq!(make_fs_slug_year("Avatar: The Last Airbender", 2005), "avatar the last airbender (2005)");
///
/// # Panics
///
/// Panics if `input` is not ASCII.
pub fn make_fs_slug_year(title: &str, year: i32) -> String {
	let words = split_normalized_words(title);
	let (_, words) = split_leading_article(words);

	Itertools::intersperse(words, " ".to_string())
		.chain([format!(" ({year})")])
		.collect()
}

/// Normalize a filesystem name
///
///     use flix_model::text::normalize_fs_name;
///
///     assert_eq!(normalize_fs_name("Matrix (1999)"), "matrix (1999)");
///     assert_eq!(normalize_fs_name("Marvel's Agents of SHIELD (2013)"), "marvels agents of shield (2013)");
///     assert_eq!(normalize_fs_name("Avatar The Last Airbender (2005)"), "avatar the last airbender (2005)");
pub fn normalize_fs_name(input: &str) -> String {
	let chars = input.split_ascii_whitespace().map(|s| {
		let chars = s
			.chars()
			.filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '(' || *c == ')')
			.map(|c| c.to_ascii_lowercase());

		if s.len() > 4
			&& s.len().is_multiple_of(2)
			&& chars.clone().tuples().all(|(l, r)| l != '.' && r == '.')
		{
			// Collapse acronym
			chars.tuples().map(|(l, _)| l).collect()
		} else {
			chars.collect()
		}
	});
	Itertools::intersperse(chars, " ".to_string()).collect()
}

/// Convert a media title to a url compatible string
///
///     use flix_model::text::make_web_slug;
///
///     assert_eq!(make_web_slug("The Matrix"), "matrix");
///     assert_eq!(make_web_slug("Marvel's Agents of S.H.I.E.L.D."), "marvels-agents-of-shield");
///     assert_eq!(make_web_slug("Avatar: The Last Airbender"), "avatar-the-last-airbender");
///
/// # Panics
///
/// Panics if `input` is not ASCII.
pub fn make_web_slug(title: &str) -> String {
	let words = split_normalized_words(title);
	let (_, words) = split_leading_article(words);

	Itertools::intersperse(words, "-".to_string()).collect()
}

/// Convert a media title and year to a url compatible string
///
///     use flix_model::text::make_web_slug_year;
///
///     assert_eq!(make_web_slug_year("The Matrix", 1999), "matrix-1999");
///     assert_eq!(make_web_slug_year("Marvel's Agents of S.H.I.E.L.D.", 2013), "marvels-agents-of-shield-2013");
///     assert_eq!(make_web_slug_year("Avatar: The Last Airbender", 2005), "avatar-the-last-airbender-2005");
///
/// # Panics
///
/// Panics if `input` is not ASCII.
pub fn make_web_slug_year(title: &str, year: i32) -> String {
	let words = split_normalized_words(title);
	let (_, words) = split_leading_article(words);

	Itertools::intersperse(words, "-".to_string())
		.chain([format!("-{year}")])
		.collect()
}
