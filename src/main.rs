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
				", AKA "
				strong { "obiwac" }
				"."
				br;
				"I'm a Belgian open-source enthusiast who likes dogs and beer 🍺"
			}
			p { "Here are a few of my projects:" }
			.things {
				.thing {
					.labeled-img {
						img src="https://user-images.githubusercontent.com/11079650/155240444-53454627-84f0-4a52-81aa-9eb60f8770e8.png";
						div {
							h2 { "aquaBSD" }
						}
					}
					p { "OS forked from FreeBSD geared towards general users." }
				}
				.thing {
					.labeled-img {
						// iframe src="https://drakeerv.github.io/js-minecraft-clone/episodes/episode-5/index.html" {}
						img src="https://github.com/obiwac/python-minecraft-clone/blob/master/eyecandy/creeper.png?raw=true";
						div {
							h2 { "MCPY" }
						}
					}
					p { "Minecraft clone written in Python. Video tutorial series on 3D graphics programming." }
				}
				.thing {
					.labeled-img {
						img src="https://github.com/karwa-org/karwa2023/blob/main/logo.png?raw=true";
						div {
							h2 { "KARWa '23" }
						}
					}
					p { "Francophone algorithmics contest. Jointly organized by Louvain-li-Nux (in Louvain-la-Neuve) and CPUMons (in Mons)." }
				}
				.thing {
					.labeled-img {
						img src="https://github.com/obiwac/bfm/raw/main/images/naive.gif";
						div {
							h2 { "BFM" }
						}
					}
					p {
						"Big F'ing Matrix. FEM/FEA C library ("
						code { "libbfm" }
						") with Python bindings ("
						code { "pybfm" }
						") for use as an educational tool. Alex and I made this for LEPL1110."
					}
				}
				.thing {
					.labeled-img {
						img src="https://github.com/obiwac/lln-gamejam-2023/raw/main/eyecandy/obamatriangle.jpg";
						div {
							h2 { "LLN 2023" }
						}
					}
					p { "Submission for the 2023 Louvain-li-Nux gamejam. AKA Alexis and I's first foray into Vulkan and Rust. AKA Obamatriangle." }
				}
				.thing {
					.labeled-img {
						img src="https://github.com/obiwac/lln-gamejam-2022/raw/main/eyecandy/volcano-look.png";
						div {
							h2 { "LLN 2022" }
						}
					}
					p { "Submission for the 2022 Louvain-li-Nux gamejam. Pure C11, X11, 7/11." }
				}
				.thing {
					.labeled-img {
						img src="https://github.com/obiwac/x-compositing-wm/raw/main/pics/screenshot1.png";
						div {
							h2 { "x-compositing-wm" }
						}
					}
					p { "Extremely basic X11 compositing window manager written in C with Xlib and OpenGL." }
				}
				.thing {
					.labeled-img {
						img src="https://github.com/NovAti0n/MOOdle/raw/main/eyecandy/paturage.png";
						div {
							h2 { "MOOdle" }
						}
					}
					p { "Advanced cow visualization tool." }
				}
				.thing {
					.labeled-img {
						img src="https://github.com/NovAti0n/GDPR-presentation/raw/main/screenshot.png";
						div {
							h2 { "GDPR" }
						}
					}
					p { "Interactive GDPR presentation Noa and I made in English class in highschool." }
				}
			}
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
