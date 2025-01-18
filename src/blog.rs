use maud::{html, Markup, PreEscaped};
use crate::common::{include_md, include_static_unsafe, Markdown, relative, include_static};
use crate::base::base;
use crate::social::social;
use rocket::route::Outcome;

fn blog_tag(key: &str, val: &str) -> Markup {
	html! {
		.blog-tag {
			b { (key) }
			(val)
		}
	}
}

pub struct Blog {
	route: &'static str,
	title: &'static str,
	descr: &'static str,
	reading_time: u32,
	date: &'static str,
	content: Markdown<&'static str>,
}

impl Blog {
	fn render(&self) -> Markup {
		let schema = format!(r#"{{
			"@context": "http://schema.org",
			"@type": "Article",
			"@id": "{{#}}article",
			"name": "{}",
			"author": "Aymeric Wibo"
		}}"#, self.title);

		base(self.title, PreEscaped(&schema), html! {
			a.go-back href="/" {
				(include_static!("/icons/back.svg"))
				p { "Main page" }
			}
			.blog-container {
				h1.blog-title { (self.title) }
				(blog_tag("Reading time:", &format!("{} min", self.reading_time)))
				(blog_tag("Date published:", self.date))
				hr;
				(self.content)
				.socials {
					(social("obiwac@freebsd.org", "mailto:obiwac@freebsd.org", include_static!("/icons/fbsd.svg")))
				}
			}
		})
	}

	pub fn render_entry(&self) -> Markup {
		html! {
			.blog-entry {
				h2 {
					a.link href=(self.route) { (self.title) }
				}
				p { (self.descr) }
				(blog_tag("Reading time:", &format!("{} min", self.reading_time)))
				(blog_tag("Date published:", self.date))
			}
		}
	}
}

pub const BLOGS: &'static [&'static Blog] = &[
	&Blog {
		route: "/s0ix",
		title: "Modern standby on FreeBSD (S0ix) ⚡",
		descr: "Overview and notes for implementing S0ix on FreeBSD, a power-saving feature on modern laptops which superseeds the previous ACPI S3 sleep state.",
		reading_time: 6,
		date: "1/11/2024",
		content: include_md!("/blog/s0ix.md"),
	},
	&Blog {
		route: "/fprint",
		title: "Biometric authentication on FreeBSD with fingerprint scanners 🔑",
		descr: "Guide on setting up fingerprint scanners on FreeBSD as a means of biometric authentication. Goes over the general software architecture and a few use cases.",
		reading_time: 5,
		date: "12/10/2024",
		content: include_md!("/blog/fprint.md"),
	},
];

pub fn blog_routes() -> Vec<rocket::Route> {
	BLOGS.iter().map(|blog| {
		let handler = for<'r, 'x> move |req: &'r rocket::Request<'x>, _: rocket::Data<'r>| -> rocket::route::BoxFuture<'r> {
			Outcome::from(req, blog.render()).pin()
		};

		rocket::route::Route::new(rocket::http::Method::Get, blog.route, handler)
	}).collect()
}
