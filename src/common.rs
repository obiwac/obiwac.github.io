use maud::{Markup, PreEscaped, Render};
use pulldown_cmark::{html, Parser};

macro_rules! relative {
	($path:expr) => {
		concat!(env!("CARGO_MANIFEST_DIR"), $path)
	};
}

macro_rules! include_static_unsafe {
	($path:expr) => {
		include_str!(relative!(concat!("/public", $path)))
	};
}

macro_rules! include_static {
	($path:expr) => {
		PreEscaped(include_static_unsafe!($path))
	};
}

macro_rules! include_css {
	($path:expr) => {
		PreEscaped(
			Minifier::default()
				.minify(include_static_unsafe!($path), Level::Three)
				.unwrap(),
		)
	};
}

macro_rules! include_md {
	($path:expr) => {
		Markdown(include_static_unsafe!($path))
	};
}

pub(crate) use {include_css, include_md, include_static, include_static_unsafe, relative};

pub struct Markdown<T>(pub T);

impl<T: AsRef<str>> Render for Markdown<T> {
	fn render(&self) -> Markup {
		let mut unsafe_html = String::new();
		let parser = Parser::new_ext(self.0.as_ref(), pulldown_cmark::Options::ENABLE_TABLES);

		// Preprocessor to highlight syntax in code blocks.

		let mut parser = cmark_syntax::SyntaxPreprocessor::new(parser);

		// Preprocessor to add "link" class to anchors.

		let mut in_link = false;
		let mut link_code = false;
		let mut link_url: Option<pulldown_cmark::CowStr> = None;
		let mut link_text: Option<pulldown_cmark::CowStr> = None;
		let mut new_parser: Vec<pulldown_cmark::Event> = Vec::new();

		while let Some(event) = parser.next() {
			match event {
				pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
					link_type: _,
					dest_url,
					title: _,
					id: _,
				}) => {
					link_url = Some(dest_url);
					in_link = true;
				}
				pulldown_cmark::Event::Text(ref text) | pulldown_cmark::Event::Code(ref text) => {
					if in_link {
						link_code = matches!(event, pulldown_cmark::Event::Code(_));
						link_text = Some(text.clone());
					} else {
						new_parser.push(event);
					}
				}
				pulldown_cmark::Event::End(pulldown_cmark::TagEnd::Link) => {
					assert!(in_link);
					in_link = false;

					let link_text = link_text.clone().unwrap().to_string();
					let inner_html = if link_code {
						format!("<code>{}</code>", link_text)
					} else {
						link_text
					};
					let html = format!(
						"<a class=\"link\" href=\"{}\">{}</a>",
						link_url.clone().unwrap(),
						inner_html
					);

					new_parser.push(pulldown_cmark::Event::InlineHtml(pulldown_cmark::CowStr::Boxed(
						html.into(),
					)));
				}
				_ => {
					assert!(!in_link);
					new_parser.push(event)
				}
			}
		}

		// Write out unsafe HTML.

		html::push_html(&mut unsafe_html, new_parser.into_iter());

		// Sanitize unsafe HTML.

		let safe = ammonia::Builder::default()
			.add_allowed_classes("a", &["link"])
			.add_tags(vec!["iframe"])
			.add_tag_attributes(
				"iframe", &["class", "src", "width", "height", "frameborder"],
			)
			.add_allowed_classes("span", &[
				"glyph",
				"literal",
				"identifier",
				"special-identifier",
				"strong-identifier",
				"keyword",
				"comment",
			])
			.clean(&unsafe_html)
			.to_string();

		PreEscaped(safe)
	}
}
