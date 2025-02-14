use maud::{Markup, PreEscaped, Render};
use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag, TagEnd};

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
		let parser = Parser::new_ext(self.0.as_ref(), Options::ENABLE_TABLES);

		// Preprocessor to highlight syntax in code blocks.

		let mut parser = cmark_syntax::SyntaxPreprocessor::new(parser);

		// Custom preprocessing.

		let mut in_link = false;
		let mut link_url: Option<CowStr> = None;
		let mut link_text: String = String::new();
		let mut new_parser: Vec<Event> = Vec::new();

		let mut in_table = false;

		while let Some(event) = parser.next() {
			match event {
				// Add the "link" class to anchors.
				Event::Start(Tag::Link {
					link_type: _,
					dest_url,
					title: _,
					id: _,
				}) => {
					link_url = Some(dest_url);
					in_link = true;
					link_text = String::new();
				}
				Event::End(TagEnd::Link) => {
					assert!(in_link);
					in_link = false;

					let inner_html = link_text.clone().to_string();
					let html = format!(
						"<a class=\"link\" href=\"{}\">{}</a>",
						link_url.clone().unwrap(),
						inner_html
					);

					new_parser.push(Event::InlineHtml(CowStr::Boxed(html.into())));
				}

				Event::Text(ref text) if in_link => {
					link_text.push_str(text);
				}
				Event::Code(ref text) if in_link => {
					link_text.push_str(format!("<code>{}</code>", text).as_str());
				}

				// Wrap tables with a div.
				Event::Start(Tag::Table(alignment)) => {
					assert!(!in_table); // Can't have a table in a table.
					in_table = true;
					new_parser.push(Event::Html(CowStr::Borrowed("<div class=\"table\">")));
					new_parser.push(Event::Start(Tag::Table(alignment)));
				}
				Event::End(TagEnd::Table) => {
					assert!(in_table);
					in_table = false;
					new_parser.push(Event::End(TagEnd::Table));
					new_parser.push(Event::Html(CowStr::Borrowed("</div>")));
				}

				// Regular events.
				_ => {
					assert!(!in_link);
					new_parser.push(event);
				}
			}
		}

		// Write out unsafe HTML.

		html::push_html(&mut unsafe_html, new_parser.into_iter());

		// Sanitize unsafe HTML.

		let safe = ammonia::Builder::default()
			.add_allowed_classes("a", &["link"])
			.add_allowed_classes("span", &[
				"glyph",
				"literal",
				"identifier",
				"special-identifier",
				"strong-identifier",
				"keyword",
				"comment",
			])
			.add_allowed_classes("div", &["table"])
			.add_tag_attributes("div", &["style"])
			.clean(&unsafe_html)
			.to_string();

		PreEscaped(safe)
	}
}
