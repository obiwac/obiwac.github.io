use maud::{html, Markup, PreEscaped};
use crate::common::{include_md, include_static_unsafe, Markdown, relative, include_static};
use crate::base::base;
use crate::social::social;
use rocket::route::Outcome;

struct Blog {
	route: &'static str,
	title: &'static str,
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
			"author": "Aymeric Wibo",
		}}"#, self.title);

		base(PreEscaped(&schema), html! {
			a.go-back href="/" {
				(include_static!("/icons/back.svg"))
				p { "Main page" }
			}
			.blog-container {
				h1.blog-title { (self.title) }
				.blog-tag {
					b { "Reading time:" }
					(format!("{} min", self.reading_time))
				}
				.blog-tag {
					b { "Date published:" }
					(self.date)
				}
				hr;
				(self.content)
				.socials {
					(social("obiwac@freebsd.org", "mailto:obiwac@freebsd.org", include_static!("/icons/fbsd.svg")))
				}
			}
		})
	}
}

pub fn blog_routes() -> Vec<rocket::Route> {
	let blogs: &'static [&'static Blog] = &[
		&Blog {
			route: "/s0ix",
			title: "Modern standby on FreeBSD (S0ix)",
			reading_time: 0,
			date: "idk",
			content: include_md!("/blog/s0ix.md"),
		},
		&Blog {
			route: "/fprint",
			title: "Biometric authentication on FreeBSD with fingerprint scanners 🔑",
			reading_time: 5,
			date: "12/10/2024",
			content: include_md!("/blog/fprint.md"),
		},
	];

	blogs.iter().map(|blog| {
		let handler = for<'r, 'x> move |req: &'r rocket::Request<'x>, _: rocket::Data<'r>| -> rocket::route::BoxFuture<'r> {
			Outcome::from(req, blog.render()).pin()
		};

		rocket::route::Route::new(rocket::http::Method::Get, blog.route, handler)
	}).collect()
}
