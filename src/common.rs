use maud::{Markup, PreEscaped, Render};
use pulldown_cmark::{Parser, html};

macro_rules! relative {
	($path: expr) => (concat!(env!("CARGO_MANIFEST_DIR"), $path))
}

macro_rules! include_static {
	($path: expr) => (include_str!(relative!(concat!("/public", $path))))
}

macro_rules! include_static_unsafe {
	($path: expr) => (PreEscaped(include_static!($path)))
}

macro_rules! include_css {
	($path: expr) => (PreEscaped(Minifier::default().minify(include_static!($path), Level::Three).unwrap()))
}

macro_rules! include_md {
	($path: expr) => (Markdown(include_static_unsafe!($path)))
}

pub(crate) use relative;
pub(crate) use include_static;
pub(crate) use include_static_unsafe;
pub(crate) use include_css;
pub(crate) use include_md;

struct Markdown<T: AsRef<str>>(T);

impl <T: AsRef<str>> Render for Markdown<T> {
	fn render(&self) -> Markup {
		let mut unsafe_html = String::new();
		let parser = Parser::new(self.0.as_ref());

		html::push_html(&mut unsafe_html, parser);

		let safe = ammonia::clean(&unsafe_html);
		PreEscaped(safe)
	}
}
