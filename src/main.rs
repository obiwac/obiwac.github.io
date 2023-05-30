#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
extern crate maud;
extern crate css_minify;

use maud::{html, Markup, DOCTYPE, PreEscaped};
use rocket::fs::FileServer;
use css_minify::optimizations::{Minifier, Level};

macro_rules! relative {
	($path: expr) => (concat!(env!("CARGO_MANIFEST_DIR"), $path))
}

macro_rules! include_static {
	($path: expr) => (include_str!(relative!(concat!("/public", $path))))
}

macro_rules! include_static_unsafe {
	($path: expr) => (PreEscaped(include_static!($path)))
}

macro_rules! include_css {
	($path: expr) => (PreEscaped(Minifier::default().minify(include_static!($path), Level::Three).unwrap()))
}

fn base(schema: &str, content: Markup) -> Markup {
	html! {
		(DOCTYPE)

		html lang="en" {
			head {
				meta charset="UTF-8"; // must be in the first 1024 bytes of the document
				meta name="description" content="Personal website for Aymeric Wibo"; // can't be longer than 275 characters as per Google's 2017 limit on the SERP
				meta name="viewport" content="width=device-width,initial-scale=1";
				meta name="robots" content="index,follow";
				meta name="theme-color" content="#000000";

				link rel="icon" type="image/png" href="/public/icons/me.png";
				link rel="manifest" href="manifest.json";

				// Apple PWA stuff

				meta name="apple-mobile-web-app-capable" content="yes";
				meta name="apple-mobile-web-app-status-bar-style" content="black-translucent";
				meta name="apple-mobile-web-app-title" content="Aymeric Wibo";

				// TODO keywords, google-site-verification, apple-touch-startup-image

				title { "Aymeric Wibo" }
				script type="application/ld+json" { (PreEscaped(schema)) }

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

// these are separate components, so I can easily update links

enum Person {
	Noa, Alexis, Alex, Drakeerv, Juk, Brichant, Aless
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
			Person::Aless => a.link href="https://github.com/akialess" { "Aless" },
		}
	}
}

// homepage

fn thing(title: &'static str, link: &'static str, magic: bool, img_src: &'static str, descr: Markup) -> Markup {
	let alt: &str = &(title.to_owned() + " thumbnail");

	html! {
		.thing {
			.labeled-img {
				img alt=(alt) src=(img_src);
				div {
					div {
						h2 { (title) }
						@if magic {
							(include_static_unsafe!("/icons/magic.svg"))
						}
					}
				}
			}
			p { (descr) }
			a.learn-more href=(link) {
				(include_static_unsafe!("/icons/arrow.svg"))
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

const MCPY_IMG_SRC: &str = "/public/thumbnails/mcpy.png";
const MOODLE_IMG_SRC: &str = "/public/thumbnails/moodle.png";
const GDPR_IMG_SRC: &str = "/public/thumbnails/gdpr.png";

#[get("/")]
fn index() -> Markup {
	base(include_static!("/schema/me.json"), html! {
		.container {
			header role="banner" {
				center {
					h1 { "Hey! 👋" }
				}
				.socials {
					(social("awibo", "https://www.linkedin.com/in/awibo", include_static_unsafe!("/icons/linkedin.svg")))
					(social("@obiwac", "https://www.github.com/obiwac", include_static_unsafe!("/icons/gh.svg")))
					(social("obiwac@gmail.com", "mailto:obiwac@gmail.com", include_static_unsafe!("/icons/email.svg")))
					(social("obiwac@freebsd.org", "mailto:obiwac@freebsd.org", include_static_unsafe!("/icons/fbsd.svg")))
					(social("obiwac", "https://youtube.com/obiwac", include_static_unsafe!("/icons/youtube.svg")))
					(social("obiwac#7599", "https://discord.com/users/305047157197504522", include_static_unsafe!("/icons/discord.svg")))
				}
			}
			main role="main" {
				p {
					"My name is "
					strong { "Aymeric Wibo" }
					" (AKA "
					strong { "obiwac" }
					", no relation to Obi-Wan). I'm a Belgian open-source enthusiast who likes dogs and beer 🍺 Here are some of my projects - those which have a "
					span.inline-svg {
						(include_static_unsafe!("/icons/magic.svg"))
					}
					" next to their name are interactive experiences:"
				}
				.things {
					(thing("aquaBSD", "https://github.com/inobulles/aquabsd/releases", false, "/public/thumbnails/aquabsd.png", html! {
						"OS forked from FreeBSD geared towards general users. Includes a full DE, app distribution system, and network device sharing."
					}))

					(thing("MCPY", "/mcpy", true, MCPY_IMG_SRC, html! {
						"Video tutorial series on 3D graphics programming with OpenGL, where I write a Minecraft clone in Python."
					}))

					(thing("BFM", "https://github.com/obiwac/bfm", false, "/public/thumbnails/bfm.png", html! {
						"Big F'ing Matrix. FEM/FEA C library ("
						code { "libbfm" }
						") with Python bindings ("
						code { "pybfm" }
						") for use as an educational tool. "
						(person(Person::Alex))
						" and I made this for LEPL1110."
					}))

					(thing("KARWa '23", "https://www.linkedin.com/posts/louvain-li-nux_algo-algorithmes-programmingcontest-activity-7054432800577306624-CR6L?utm_source=share&utm_medium=member_desktop", false, "/public/thumbnails/karwa.png", html! {
						"Francophone algorithmics contest. Jointly organized by Louvain-li-Nux (in Louvain-la-Neuve) and CPUMons (in Mons)."
					}))

					(thing("MOOdle", "/moodle", true, MOODLE_IMG_SRC, html! {
						"Advanced cow visualization tool."
					}))

					(thing("GDPR", "/gdpr", true, GDPR_IMG_SRC, html! {
						"Interactive GDPR presentation "
						(person(Person::Noa))
						" and I made in English class in highschool."
					}))

					(thing("LLN '23", "https://github.com/obiwac/lln-gamejam-2023", false, "/public/thumbnails/lln23.png", html! {
						"Submission for the 2023 Louvain-li-Nux gamejam. AKA "
						(person(Person::Alexis))
						" and "
						(person(Person::Aless))
						" and I's first foray into Vulkan and Rust. AKA Obamatriangle."
					}))

					(thing("LLN '22", "https://github.com/obiwac/lln-gamejam-2022", false, "/public/thumbnails/lln22.png", html! {
						"Submission for the 2022 Louvain-li-Nux gamejam. Pure C11. Pure X11. Pure 7/11."
					}))

					(thing("Compositing WM", "https://github.com/obiwac/x-compositing-wm", false, "/public/thumbnails/x.png", html! {
						"Extremely basic X11 compositing window manager written in C with Xlib and OpenGL."
					}))
				}
			}
			footer role="contentinfo" {
				p {
					"This page was made possible thanks to "
					a.link href="https://rocket.rs" { "Rocket.rs" }
					" and "
					a.link href="https://maud.lambda.xyz" { "Maud" }
					"! Fun fact: this site's source doesn't have a single line of the godforsaken language known as HTML in it. It does have some JS on some pages though (not this one), so count that as an L if you want."
				}
				.socials {
					(social("Source code", "https://github.com/obiwac/obiwac.github.io", include_static_unsafe!("/icons/gh.svg")))
				}
			}
		}
	})
}

// explanation pages

fn explanation_page(title: &'static str, img_src: &'static str, descr: Markup, exhibit: Markup) -> Markup {
	let schema = format!(r#"{{
		"@context": "http://schema.org",
		"@type": "Article",
		"@id": "{{#}}article",
		"name": "{}",
		"author": "Aymeric Wibo",
		"image": "{}"
	}}"#, title, img_src);

	base(&schema, html! {
		.explanation-container {
			.explanation #article {
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
	explanation_page("MCPY ⛏️", MCPY_IMG_SRC, html! {
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
			(social("Playlist", "https://www.youtube.com/watch?v=fWkbIOna6RA&list=PL6_bLxRDFzoKjaa3qCGkwR5L_ouSreaVP", include_static_unsafe!("/icons/youtube.svg")))
			(social("Source code", "https://github.com/obiwac/python-minecraft-clone", include_static_unsafe!("/icons/gh.svg")))
			(social("Full demo", "https://drakeerv.github.io/js-minecraft-clone/", include_static_unsafe!("/icons/link.svg")))
		}
	}, html! {
		iframe title="Drakeerv's port of MCPY to the browser" src="https://drakeerv.github.io/js-minecraft-clone/episodes/episode-11/index.html" loading="lazy";
	})
}

#[get("/moodle")]
fn moodle() -> Markup {
	explanation_page("MOOdle 🐮", MOODLE_IMG_SRC, html! {
		p {
			"Advanced cow visualization tool. This was originally made with my friends "
			(person(Person::Noa))
			" and "
			(person(Person::Alexis))
			" for a university course, using our proprietary VirtualRanch™ technology."
		}
		p {
			"Notice the subtle and difficult to understand play on words on the popular learning platform "
			a.link href="https://moodle.org/" { "Moodle" }
			"."
		}
		p {
			"You can try out the full version by clicking the link. "
			strong { "Content warning" }
			": French. Sensitive viewers are advised to look away."
		}
		.socials {
			(social("Source code", "https://github.com/novati0n/moodle", include_static_unsafe!("/icons/gh.svg")))
			(social("Full version", "https://moodle.novation.dev", include_static_unsafe!("/icons/link.svg")))
		}
	}, html! {
		// settings (because we're not attached to a full webapp anymore)

		script {
			(PreEscaped(r#"
				var invert_gravity = false
				var cow_speed = 2

				var data = {
					"Holstein": 20,
					"Jersey": 5,
					"Blanc Bleu Belge": 10,
				}
			"#))
		}

		// shaders

		script #vert-shader type="x-shader/x-vertex" { (include_static_unsafe!("/moodle/vert.glsl")) }
		script #frag-shader type="x-shader/x-fragment" { (include_static_unsafe!("/moodle/frag.glsl")) }

		// models

		script src="/public/moodle/models/paturage.js" defer {}
		script src="/public/moodle/models/holstein.js" defer {}
		script src="/public/moodle/models/jersey.js" defer {}
		script src="/public/moodle/models/bbb.js" defer {}

		// actual paturage

		canvas #paturage title="A herd of cows having the time of their lives... in captivity" width="800px" height="500px" onclick="paturage.click()" {}
		script src="/public/moodle/paturage.js" defer {}
	})
}

#[get("/gdpr")]
fn gdpr() -> Markup {
	explanation_page("GDPR 🤓", GDPR_IMG_SRC, html! {
		p {
			"Interactive (try it out right here - don't worry, we don't use cookies 😉) GDPR presentation my friend "
			(person(Person::Noa))
			" and I made in English class in highschool. As such, some parts may be written in French, as this was an English class in "
			a.link href="https://en.wikipedia.org/wiki/Wallonia" { "Wallonia" }
			" 🇧🇪"
		}
		p { "There used to be a (extremely poorly secured 😄) database system to record quiz/survey answers, but that's now offline." }
		p { "Also, the code is very not pretty. We wrote this in like 2 days, certainly not with the intention of further maintaining it." }
		p {
			"In memorandum "
			(person(Person::Brichant))
			" (don't press "
			code { "Ctrl+Alt+B" }
			") ❤️"
		}
		.socials {
			(social("Source code", "https://github.com/novati0n/gdpr-presentation", include_static_unsafe!("//icons/gh.svg")))
			(social("Full version", "https://novation.dev/GDPR-presentation", include_static_unsafe!("/icons/link.svg")))
		}
	}, html! {
		iframe title="The GDPR presentation in question" src="https://novation.dev/GDPR-presentation" loading="lazy";
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
