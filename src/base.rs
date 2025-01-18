use maud::{html, DOCTYPE, Markup, PreEscaped};
use crate::common::{relative, include_static_unsafe, include_css};
use css_minify::optimizations::{Minifier, Level};

pub fn base(title: &str, schema: PreEscaped<&str>, content: Markup) -> Markup {
	html! {
		(DOCTYPE)

		html lang="en" {
			head {
				meta charset="UTF-8"; // must be in the first 1024 bytes of the document
				meta name="description" content="Personal website for Aymeric Wibo"; // can't be longer than 275 characters as per Google's 2017 limit on the SERP
				meta name="viewport" content="width=device-width,initial-scale=1";
				meta name="robots" content="index,follow";
				meta name="google-site-verification" content="fAAF9QVbOi5rD1tThBbfzVtfhyAFbl4iN2LR42G67TI";
				meta name="theme-color" content="#000000";

				link rel="icon" type="image/png" href="/public/icons/me.png";
				link rel="manifest" href="manifest.json";

				// Apple PWA stuff

				meta name="apple-mobile-web-app-capable" content="yes";
				meta name="apple-mobile-web-app-status-bar-style" content="black-translucent";
				meta name="apple-mobile-web-app-title" content=(title);

				// TODO keywords, apple-touch-startup-image

				title { (title) }
				script type="application/ld+json" { (schema) }

				// link rel="stylesheet" type="text/css" href="/public/main.css";

				style {
					(include_css!("/main.css"))
				}
			}

			body {
				(content)
			}
		}
	}
}

