use maud::{html, Markup, PreEscaped};
use crate::common::{include_md, include_static_unsafe, Markdown, relative, include_static};
use crate::base::base;
use crate::social::social;

fn blog_page(title: &'static str, reading_time: u32, date: &'static str, content: Markdown<&str>) -> Markup {
	let schema = format!(r#"{{
		"@context": "http://schema.org",
		"@type": "Article",
		"@id": "{{#}}article",
		"name": "{}",
		"author": "Aymeric Wibo",
	}}"#, title);

	base(PreEscaped(&schema), html! {
		a.go-back href="/" {
			(include_static!("/icons/back.svg"))
			p { "Main page" }
		}
		.blog-container {
			h1.blog-title { (title) }
			.blog-tag {
				b { "Reading time:" }
				(format!("{} min", reading_time))
			}
			.blog-tag {
				b { "Date published:" }
				(date)
			}
			hr;
			(content)
			.socials {
				(social("obiwac@freebsd.org", "mailto:obiwac@freebsd.org", include_static!("/icons/fbsd.svg")))
			}
		}
	})
}

#[get("/s0ix")]
pub fn s0ix() -> Markup {
	blog_page("Modern standby on FreeBSD (S0ix)", 0, "idk", include_md!("/blog/s0ix.md"))
}

#[get("/fprint")]
pub fn fprint() -> Markup {
	blog_page("Biometric authentication on FreeBSD with fingerprint scanners 🔑", 5, "12/10/2024", include_md!("/blog/fprint.md"))
}

pub fn blog_routes() -> Vec<rocket::Route> {
	routes![s0ix, fprint]
}
