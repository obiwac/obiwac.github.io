use maud::{html, Markup, PreEscaped};
use crate::common::{include_md, include_static_unsafe, Markdown, relative};
use crate::base::base;

fn blog_page(title: &'static str, content: Markdown<&str>) -> Markup {
	let schema = format!(r#"{{
		"@context": "http://schema.org",
		"@type": "Article",
		"@id": "{{#}}article",
		"name": "{}",
		"author": "Aymeric Wibo",
	}}"#, title);

	base(PreEscaped(&schema), html! {
		(content)
	})
}

#[get("/s0ix")]
pub fn s0ix() -> Markup {
	blog_page("Modern standby on FreeBSD (S0ix)", include_md!("/blog/s0ix.md"))
}
