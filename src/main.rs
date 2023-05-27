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

// homepage

fn thing(title: &'static str, link: &'static str, magic: bool, img_src: &'static str, descr: Markup) -> Markup {
	html! {
		.thing {
			.labeled-img {
				img src=(img_src);
				div {
					div {
						h2 { (title) }
						@if magic {
							(include_static!("/static/icons/magic.svg"))
						}
					}
				}
			}
			p { (descr) }
			a.learn-more href=(link) {
				(include_static!("/static/icons/arrow.svg"))
				p { "Learn more" }
			}
		}
	}
}

fn social(handle: &'static str, link: &'static str, icon: PreEscaped<&str>) -> Markup {
	html! {
		a.social href=(link) {
			(icon)
			p { (handle) }
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
				", AKA "
				strong { "obiwac" }
				". I'm a Belgian open-source enthusiast who likes dogs and beer 🍺 "
				"My socials are at the bottom of this page if you'd like to contact me!"
			}
			p {
				"Here are some of my projects - those which have a "
				span.inline-svg {
					(include_static!("/static/icons/magic.svg"))
				}
				" next to their name are interactive experiences:"
			}
			.things {
				(thing("aquaBSD", "/aquabsd", false, "https://user-images.githubusercontent.com/11079650/155240444-53454627-84f0-4a52-81aa-9eb60f8770e8.png", html! {
					"OS forked from FreeBSD geared towards general users. Includes a full DE, app distribution system, and network device sharing."
				}))

				(thing("MCPY", "/mcpy", true, "https://github.com/obiwac/python-minecraft-clone/blob/master/eyecandy/creeper.png?raw=true", html! {
					"Minecraft clone written in Python. Video tutorial series on 3D graphics programming."
				}))

				(thing("BFM", "/bfm", true, "https://github.com/obiwac/bfm/raw/main/images/naive.gif", html! {
					"Big F'ing Matrix. FEM/FEA C library ("
					code { "libbfm" }
					") with Python bindings ("
					code { "pybfm" }
					") for use as an educational tool. Alex and I made this for LEPL1110."
				}))

				(thing("KARWa '23", "/karwa", false, "https://github.com/karwa-org/karwa2023/blob/main/logo.png?raw=true", html! {
					"Francophone algorithmics contest. Jointly organized by Louvain-li-Nux (in Louvain-la-Neuve) and CPUMons (in Mons)."
				}))

				(thing("MOOdle", "/moodle", true, "https://github.com/NovAti0n/MOOdle/raw/main/eyecandy/paturage.png", html! {
					"Advanced cow visualization tool."
				}))

				(thing("GDPR", "/gdpr", true, "https://github.com/NovAti0n/GDPR-presentation/raw/main/screenshot.png", html! {
					"Interactive GDPR presentation Noa and I made in English class in highschool."
				}))

				(thing("LLN '23", "https://github.com/obiwac/lln-gamejam-2023", false, "https://github.com/obiwac/lln-gamejam-2023/raw/main/eyecandy/obamatriangle.jpg", html! {
					"Submission for the 2023 Louvain-li-Nux gamejam. AKA Alexis and I's first foray into Vulkan and Rust. AKA Obamatriangle."
				}))

				(thing("LLN '22", "https://github.com/obiwac/lln-gamejam-2022", false, "https://github.com/obiwac/lln-gamejam-2022/raw/main/eyecandy/volcano-look.png", html! {
					"Submission for the 2022 Louvain-li-Nux gamejam. Pure C11. Pure X11. Pure 7/11."
				}))

				(thing("x-compositing-wm", "https://github.com/obiwac/x-compositing-wm", false, "https://github.com/obiwac/x-compositing-wm/raw/main/pics/screenshot1.png", html! {
					"Extremely basic X11 compositing window manager written in C with Xlib and OpenGL."
				}))
			}
			.socials {
				(social("awibo", "https://www.linkedin.com/in/awibo", include_static!("/static/icons/linkedin.svg")))
				(social("@obiwac", "https://www.github.com/obiwac", include_static!("/static/icons/gh.svg")))
				(social("obiwac@gmail.com", "mailto:obiwac@gmail.com", include_static!("/static/icons/email.svg")))
				(social("obiwac@freebsd.org", "mailto:obiwac@freebsd.org", include_static!("/static/icons/fbsd.svg")))
				(social("obiwac", "https://youtube.com/obiwac", include_static!("/static/icons/youtube.svg")))
				(social("obiwac#7599", "https://discord.com/users/305047157197504522", include_static!("/static/icons/discord.svg")))
			}
		}
	})
}

// GDPR

fn explanation_page(title: &'static str, descr: Markup, exhibit: Markup) -> Markup {
	base(html! {
		.explanation-container {
			.explanation {
				h1 { (title) }
				(descr)
			}
			.exhibit {
				(exhibit)
			}
		}
	})
}

#[get("/gdpr")]
fn gdpr() -> Markup {
	explanation_page("GDPR", html! {
		p {
			"Interactive (try it out right here - don't worry, we don't use cookies 😉) GDPR presentation my friend "
			a.link href="https://novation.dev" { "Noa" }
			" and I made in English class in highschool. As such, some parts may be written in French, as this was an English class in "
			a.link href="https://en.wikipedia.org/wiki/Wallonia" { "Wallonia" }
			"."
		}
		p { "There used to be a (extremely poorly secured 😄) database system to record quiz/survey answers, but that's now offline." }
		p { "Also, the code is very not pretty. We wrote this in like 2 days, certainly not with the intention of further maintaining it." }
		p { "In memorandum Monsieur Brichant ❤️" }
		.socials {
			(social("Source code", "https://github.com/novati0n/gdpr-presentation", include_static!("/static/icons/gh.svg")))
			(social("Full version", "https://novation.dev/GDPR-presentation", include_static!("/static/icons/link.svg")))
		}
	}, html! {
		iframe src="https://novation.dev/GDPR-presentation";
	})
}

// server stuff

#[launch]
fn rocket() -> _ {
	let rocket = rocket::build();

	rocket
		.mount("/", routes![index, gdpr])
		.mount("/public", FileServer::from(relative!("/static")))
}
