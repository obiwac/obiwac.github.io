use maud::{Markup, PreEscaped, Render};
use pulldown_cmark::{Parser, html};

macro_rules! relative {
	($path: expr) => (concat!(env!("CARGO_MANIFEST_DIR"), $path))
}

macro_rules! include_static_unsafe {
	($path: expr) => (include_str!(relative!(concat!("/public", $path))))
}

macro_rules! include_static {
	($path: expr) => (PreEscaped(include_static_unsafe!($path)))
}

macro_rules! include_css {
	($path: expr) => (PreEscaped(Minifier::default().minify(include_static_unsafe!($path), Level::Three).unwrap()))
}

macro_rules! include_md {
	($path: expr) => (Markdown(include_static_unsafe!($path)))
}

pub(crate) use relative;
pub(crate) use include_static_unsafe;
pub(crate) use include_static;
pub(crate) use include_css;
pub(crate) use include_md;

pub struct Markdown<T>(pub T);

impl<T: AsRef<str>> Render for Markdown<T> {
	fn render(&self) -> Markup {
		let mut unsafe_html = String::new();
		let parser = Parser::new(self.0.as_ref());

		// Write out unsafe HTML.

		html::push_html(&mut unsafe_html, parser.into_iter());

		// Sanitize unsafe HTML.

		let safe = ammonia::Builder::default()
			.add_allowed_classes("a", &["link"])
			.add_allowed_classes("span", &["glyph", "literal", "identifier", "special-identifier", "strong-identifier", "keyword", "comment"])
			.clean(&unsafe_html)
			.to_string();

		PreEscaped(safe)
	}
}
