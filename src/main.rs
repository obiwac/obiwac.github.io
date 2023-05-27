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
	($path: expr) => (PreEscaped(include_str!(relative!(concat!("/public", $path)))))
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
				(include_static!("/main.css"))
			}
		}

		body {
			(content)
		}
	}
}

// these are separate components, so I can easily update links

enum Person {
	Noa, Alexis, Alex, Drakeerv, Juk, Brichant
}

fn person(person: Person) -> Markup {
	html! {
		@match person {
			Person::Noa => a.link href="https://novation.dev" { "Noa" },
			Person::Alexis => a.link href="https://github.com/alexisloic21" { "Alexis" },
			Person::Alex => a.link href="https://github.com/alleyezoncode" { "Alex" },
			Person::Drakeerv => a.link href="https://github.com/drakeerv" { "@drakeerv" },
			Person::Juk => a.link href="https://github.com/jukitsu" { "@jukitsu" },
			Person::Brichant => a.link href="http://brichant.eu" { "Monsieur Brichant" },
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
							(include_static!("/icons/magic.svg"))
						}
					}
				}
			}
			p { (descr) }
			a.learn-more href=(link) {
				(include_static!("/icons/arrow.svg"))
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
			center {
				h1 { "Hey! 👋" }
			}
			.socials {
				(social("awibo", "https://www.linkedin.com/in/awibo", include_static!("/icons/linkedin.svg")))
				(social("@obiwac", "https://www.github.com/obiwac", include_static!("/icons/gh.svg")))
				(social("obiwac@gmail.com", "mailto:obiwac@gmail.com", include_static!("/icons/email.svg")))
				(social("obiwac@freebsd.org", "mailto:obiwac@freebsd.org", include_static!("/icons/fbsd.svg")))
				(social("obiwac", "https://youtube.com/obiwac", include_static!("/icons/youtube.svg")))
				(social("obiwac#7599", "https://discord.com/users/305047157197504522", include_static!("/icons/discord.svg")))
			}
			p {
				"My name is "
				strong { "Aymeric Wibo" }
				" (AKA "
				strong { "obiwac" }
				"). I'm a Belgian open-source enthusiast who likes dogs and beer 🍺 "
			}
			p {
				"Here are some of my projects - those which have a "
				span.inline-svg {
					(include_static!("/icons/magic.svg"))
				}
				" next to their name are interactive experiences:"
			}
			.things {
				(thing("aquaBSD", "https://github.com/inobulles/aquabsd/releases", false, "https://user-images.githubusercontent.com/11079650/155240444-53454627-84f0-4a52-81aa-9eb60f8770e8.png", html! {
					"OS forked from FreeBSD geared towards general users. Includes a full DE, app distribution system, and network device sharing."
				}))

				(thing("MCPY", "/mcpy", true, "https://github.com/obiwac/python-minecraft-clone/blob/master/eyecandy/creeper.png?raw=true", html! {
					"Video tutorial series on 3D graphics programming, where I write a Minecraft clone in Python."
				}))

				(thing("BFM", "https://github.com/obiwac/bfm", false, "https://github.com/obiwac/bfm/raw/main/images/naive.gif", html! {
					"Big F'ing Matrix. FEM/FEA C library ("
					code { "libbfm" }
					") with Python bindings ("
					code { "pybfm" }
					") for use as an educational tool. "
					(person(Person::Alex))
					" and I made this for LEPL1110."
				}))

				(thing("KARWa '23", "https://www.linkedin.com/posts/louvain-li-nux_algo-algorithmes-programmingcontest-activity-7054432800577306624-CR6L?utm_source=share&utm_medium=member_desktop", false, "https://github.com/karwa-org/karwa2023/blob/main/logo.png?raw=true", html! {
					"Francophone algorithmics contest. Jointly organized by Louvain-li-Nux (in Louvain-la-Neuve) and CPUMons (in Mons)."
				}))

				(thing("MOOdle", "/moodle", true, "https://github.com/NovAti0n/MOOdle/raw/main/eyecandy/paturage.png", html! {
					"Advanced cow visualization tool."
				}))

				(thing("GDPR", "/gdpr", true, "https://github.com/NovAti0n/GDPR-presentation/raw/main/screenshot.png", html! {
					"Interactive GDPR presentation "
					(person(Person::Noa))
					" and I made in English class in highschool."
				}))

				(thing("LLN '23", "https://github.com/obiwac/lln-gamejam-2023", false, "https://github.com/obiwac/lln-gamejam-2023/raw/main/eyecandy/obamatriangle.jpg", html! {
					"Submission for the 2023 Louvain-li-Nux gamejam. AKA "
					(person(Person::Alexis))
					" and I's first foray into Vulkan and Rust. AKA Obamatriangle."
				}))

				(thing("LLN '22", "https://github.com/obiwac/lln-gamejam-2022", false, "https://github.com/obiwac/lln-gamejam-2022/raw/main/eyecandy/volcano-look.png", html! {
					"Submission for the 2022 Louvain-li-Nux gamejam. Pure C11. Pure X11. Pure 7/11."
				}))

				(thing("x-compositing-wm", "https://github.com/obiwac/x-compositing-wm", false, "https://github.com/obiwac/x-compositing-wm/raw/main/pics/screenshot1.png", html! {
					"Extremely basic X11 compositing window manager written in C with Xlib and OpenGL."
				}))
			}
		}
	})
}

// explanation pages

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

#[get("/mcpy")]
fn mcpy() -> Markup {
	explanation_page("MCPY ⛏️", html! {
		p { "Video tutorial series on 3D graphics programming, where I write a Minecraft clone in Python." }
		p {
			"This page has an interactive demo (of episode 11) made in WebGL based on MCPY by "
			(person(Person::Drakeerv))
			" - it takes a little while to load (because JS is slow), but once it's loaded, you can click on it and move around like the real thing!"
		}
		p {
			"The "
			code { "community/" }
			" directory on the GitHub repo (mostly maintained by "
			(person(Person::Juk))
			" and "
			(person(Person::Drakeerv))
			") implements other cool features, such as lighting, smooth shading, and (soon) mobs!"
		}
		.socials {
			(social("Playlist", "https://www.youtube.com/watch?v=fWkbIOna6RA&list=PL6_bLxRDFzoKjaa3qCGkwR5L_ouSreaVP", include_static!("/icons/youtube.svg")))
			(social("Source code", "https://github.com/obiwac/python-minecraft-clone", include_static!("/icons/gh.svg")))
			(social("Full demo", "https://drakeerv.github.io/js-minecraft-clone/", include_static!("/icons/link.svg")))
		}
	}, html! {
		iframe src="https://drakeerv.github.io/js-minecraft-clone/episodes/episode-11/index.html" loading="lazy";
	})
}

#[get("/moodle")]
fn moodle() -> Markup {
	explanation_page("MOOdle 🐮", html! {
		p {
			"Advanced cow visualization tool. This was originally made with my friends "
			(person(Person::Noa))
			" and "
			(person(Person::Alexis))
			" for a university course."
		}
		p {
			"Notice the subtle and difficult to understand play on words on the popular learning platform "
			a.link href="https://moodle.org/" { "Moodle" }
			"."
		}
		p { "We'll host it soon (it uses Flask so we can't host it statically 😢), but in the meantime, please enjoy our proprietary VirtualRanch™ technology." }
		.socials {
			(social("Source code", "https://github.com/novati0n/moodle", include_static!("/icons/gh.svg")))
			// TODO host full version!
			// (social("Full version", "https://novation.dev/GDPR-presentation", include_static!("/icons/link.svg")))
		}
	}, html! {
		// settings (because we're not attached to a full webapp anymore)

		script {
			(PreEscaped(r#"
				var invert_gravity = false
				var cow_speed = 2

				var data = {
					"Holstein": 60,
					"Jersey": 10,
					"Blanc Bleu Belge": 20,
				}
			"#))
		}

		// shaders

		script id="vert-shader" type="x-shader/x-vertex" { (include_static!("/moodle/vert.glsl")) }
		script id="frag-shader" type="x-shader/x-fragment" { (include_static!("/moodle/frag.glsl")) }

		// models

		script src="/public/moodle/models/paturage.js" defer {}
		script src="/public/moodle/models/holstein.js" defer {}
		script src="/public/moodle/models/jersey.js" defer {}
		script src="/public/moodle/models/bbb.js" defer {}

		// actual paturage

		canvas width="800px" height="500px" id="paturage" onclick="paturage.click()" {}
		script src="/public/moodle/paturage.js" defer {}
	})
}

#[get("/gdpr")]
fn gdpr() -> Markup {
	explanation_page("GDPR 🤓", html! {
		p {
			"Interactive (try it out right here - don't worry, we don't use cookies 😉) GDPR presentation my friend "
			(person(Person::Noa))
			" and I made in English class in highschool. As such, some parts may be written in French, as this was an English class in "
			a.link href="https://en.wikipedia.org/wiki/Wallonia" { "Wallonia" }
			"."
		}
		p { "There used to be a (extremely poorly secured 😄) database system to record quiz/survey answers, but that's now offline." }
		p { "Also, the code is very not pretty. We wrote this in like 2 days, certainly not with the intention of further maintaining it." }
		p {
			"In memorandum "
			(person(Person::Brichant))
			" ❤️"
		}
		.socials {
			(social("Source code", "https://github.com/novati0n/gdpr-presentation", include_static!("//icons/gh.svg")))
			(social("Full version", "https://novation.dev/GDPR-presentation", include_static!("/icons/link.svg")))
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
		.mount("/", routes![index, mcpy, moodle, gdpr])
		.mount("/public", FileServer::from(relative!("/public")))
}
