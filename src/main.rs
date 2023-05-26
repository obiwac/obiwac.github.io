#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;

extern crate maud;
use maud::{html, Markup, DOCTYPE, PreEscaped};
use rocket::fs::FileServer;

macro_rules! relative {
	($path: expr) => (concat!(env!("CARGO_MANIFEST_DIR"), $path))
}

fn base(content: Markup) -> Markup {
	html! {
		(DOCTYPE)

		head {
			meta charset="UTF-8"; // must be in the first 1024 bytes of the document
			meta name="description" content="Personal website for Aymeric Wibo"; // can't be longer than 275 characters as per Google's 2017 limit on the SERP
			meta name="viewport" content="width=device-width,initial-scale=1";
			meta name="robots" content="index,follow";
			meta name="theme-color" content="#000000";

			link rel="icon" type="image/png" href="https://avatars.githubusercontent.com/u/81159434?s=400&u=52b722ee2247446fdb89cd4aa43d416a0ad97e14&v=4";
			link rel="manifest" href="manifest.json";

			// Apple PWA stuff

			meta name="apple-mobile-web-app-capable" content="yes";
			meta name="apple-mobile-web-app-status-bar-style" content="black-translucent";
			meta name="apple-mobile-web-app-title" content="De Bird";

			// TODO keywords, google-site-verification, apple-touch-startup-image

			title { "Aymeric Wibo" }

			// link rel="stylesheet" type="text/css" href="/public/main.css";

			style {
				(PreEscaped(include_str!(relative!("/static/main.css"))))
			}
		}

		body {
			(content)
		}
	}
}

#[get("/")]
fn index() -> Markup {
	base(html! {
		h1 { "Aymeric Wibo" }
	})
}

#[launch]
fn rocket() -> _ {
	let rocket = rocket::build();

	rocket
		.mount("/", routes![index])
		.mount("/public", FileServer::from(relative!("/static")))
}
