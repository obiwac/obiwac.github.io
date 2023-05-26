#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;

extern crate maud;
use maud::{html, Markup, DOCTYPE, PreEscaped};
use rocket::fs::FileServer;

macro_rules! relative {
	($path: expr) => (concat!(env!("CARGO_MANIFEST_DIR"), $path))
}

macro_rules! include_static {
	($path: expr) => (PreEscaped(include_str!(relative!($path))))
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
				(include_static!("/static/main.css"))
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
		.container {
			h1 { "Hey! 👋" }
			p {
				"My name is "
				strong { "Aymeric Wibo" }
				", a Belgian open source software fanatic! My current main projects are "
				a.link href="/aquabsd" { "aquaBSD" }
				", which is an OS forked from FreeBSD geared towards general users, and a "
				a.link href="/mcpy" { "video tutorial series" }
				" on 3D graphics programming in Python."
			}
			p { "Here are my socials:" }
			.socials {
				a.social href="https://www.linkedin.com/in/awibo" {
					(include_static!("/static/icons/linkedin.svg"))
					p { "awibo" }
				}
				a.social href="https://www.github.com/obiwac" {
					(include_static!("/static/icons/gh.svg"))
					p { "@obiwac" }
				}
				a.social href="mailto:obiwac@gmail.com" {
					(include_static!("/static/icons/email.svg"))
					p { "obiwac@gmail.com" }
				}
				a.social href="mailto:obiwac@freebsd.org" {
					(include_static!("/static/icons/fbsd.svg"))
					p { "obiwac@freebsd.org" }
				}
				a.social href="https://youtube.com/obiwac" {
					(include_static!("/static/icons/youtube.svg"))
					p { "obiwac" }
				}
				a.social href="https://discord.com/users/305047157197504522" {
					(include_static!("/static/icons/discord.svg"))
					p { "obiwac#7599" }
				}
			}
		}
	})
}

#[launch]
fn rocket() -> _ {
	let rocket = rocket::build();

	rocket
		.mount("/", routes![index])
		.mount("/public", FileServer::from(relative!("/static")))
}
