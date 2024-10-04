use maud::{html, Markup, PreEscaped};

pub fn social(handle: &'static str, link: &'static str, icon: PreEscaped<&str>) -> Markup {
	html! {
		a.social href=(link) {
			(icon)
			p { (handle) }
		}
	}
}
