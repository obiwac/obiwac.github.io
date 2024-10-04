use maud::{html, Markup};
use crate::common::{include_md, include_static_unsafe, Markdown, relative};

pub fn blog_page(content: Markdown<&str>) -> Markup {
	html! {
		section {
			h1 { "Blog" }
			p { "This is a blog post." }
			(content)
		}
	}
}

#[get("/s0ix")]
pub fn s0ix() -> Markup {
	blog_page(include_md!("/blog/s0ix.md"))
}
