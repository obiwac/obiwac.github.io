#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
// #![feature(decl_macro)]

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
				meta name="google-site-verification" content="fAAF9QVbOi5rD1tThBbfzVtfhyAFbl4iN2LR42G67TI";
				meta name="theme-color" content="#000000";

				link rel="icon" type="image/png" href="/public/icons/me.png";
				link rel="manifest" href="manifest.json";

				// Apple PWA stuff

				meta name="apple-mobile-web-app-capable" content="yes";
				meta name="apple-mobile-web-app-status-bar-style" content="black-translucent";
				meta name="apple-mobile-web-app-title" content="Aymeric Wibo";

				// TODO keywords, apple-touch-startup-image

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
	Noa, Alexis, Alex, Drakeerv, Juk, Brichant, Aless, Piwy,
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
			Person::Piwy => a.link href="https://github.com/Piwy-dev" { "Piwy" },
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

// Imagemagick command to resize image to 400x250: `convert -resize 400x250 input.png output.png`

const AQUABSD_IMG_SRC:   &str = "/public/thumbnails/aquabsd-small.png";
const MCPY_IMG_SRC:      &str = "/public/thumbnails/mcpy-small.png";
const BFM_IMG_SRC:       &str = "/public/thumbnails/bfm-small.png";
const KARWA_IMG_SRC:     &str = "/public/thumbnails/karwa-small.png";
const MOODLE_IMG_SRC:    &str = "/public/thumbnails/moodle-small.png";
const GDPR_IMG_SRC:      &str = "/public/thumbnails/gdpr-small.png";
const LLN24_IMG_SRC:     &str = "/public/thumbnails/lln24-small.png";
const LLN23_IMG_SRC:     &str = "/public/thumbnails/lln23-small.png";
const LLN22_IMG_SRC:     &str = "/public/thumbnails/lln22-small.png";
const X_IMG_SRC:         &str = "/public/thumbnails/x-small.png";
const _24H_VELO_IMG_SRC: &str = "/public/thumbnails/24hvelo-small.png";
const DESIGN_IMG_SRC:    &str = "/public/thumbnails/graphic-design-small.webp";

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
					(social("@obiwac", "https://github.com/obiwac", include_static_unsafe!("/icons/gh.svg")))
					(social("obiwac@gmail.com", "mailto:obiwac@gmail.com", include_static_unsafe!("/icons/email.svg")))
					(social("obiwac@freebsd.org", "mailto:obiwac@freebsd.org", include_static_unsafe!("/icons/fbsd.svg")))
					(social("obiwac", "https://youtube.com/obiwac", include_static_unsafe!("/icons/youtube.svg")))
					(social("obiwac", "https://discord.com/users/305047157197504522", include_static_unsafe!("/icons/discord.svg")))
				}
			}
			main role="main" {
				p {
					"My name is "
					strong { "Aymeric Wibo" }
					" (aka "
					strong { "obiwac" }
					"). I'm a Belgian open-source enthusiast who likes dogs and beer 🍺 Here are some of my bigger projects - those which have a "
					span.inline-svg {
						(include_static_unsafe!("/icons/magic.svg"))
					}
					" next to their name are interactive experiences."
				}
				p {
					"My interests programming-wise lie mostly in operating systems and graphics programming, but I'm also a huge public transport nerd."
				}
				.things {
					(thing("aquaBSD", "https://github.com/inobulles/aquabsd/releases", false, AQUABSD_IMG_SRC, html! {
						"OS forked from FreeBSD geared towards general users. Includes a full DE, app distribution system, and network device sharing."
					}))

					(thing("MCPY", "/mcpy", true, MCPY_IMG_SRC, html! {
						"Video tutorial series on 3D graphics programming with OpenGL, where I write a Minecraft clone in Python."
					}))

					(thing("BFM", "/bfm", true, BFM_IMG_SRC, html! {
						"Big F'ing Matrix. FEM/FEA C library ("
						code { "libbfm" }
						") with Python bindings ("
						code { "pybfm" }
						") for use as an educational tool. "
						(person(Person::Alex))
						" and I made this for LEPL1110."
					}))

					(thing("KARWa", "/karwa", false, KARWA_IMG_SRC, html! {
						"Francophone algorithmics contest. Jointly organized by Louvain-li-Nux (in Louvain-la-Neuve) and CPUMons (in Mons)."
					}))

					(thing("24h Vélo", "/24hvelo", false, _24H_VELO_IMG_SRC, html! {
						"Work done for the "
						a.link href="https://24heureslln.be" { "24h Vélo de Louvain-la-Neuve" }
						". Made a folkloric bike as well as visualization software for a giant screen on the Grand' Place."
					}))

					(thing("Graphic design", "/graphic-design", false, DESIGN_IMG_SRC, html! {
						"I like creating posters for various student events, and am generally (casually) interested in graphic design."
					}))
				}
			}
			p {
				"Here are a few more random smaller side-projects I've worked on and that I deem to be finished."
			}
			.things {
				(thing("Compositing WM", "/x-compositing-wm", false, X_IMG_SRC, html! {
					"Extremely basic X11 compositing window manager written in C with Xlib and OpenGL. A modified version is used in a helicopter simulator at the "
					a.link href="https://www.dlr.de/de/das-dlr/standorte-und-bueros/braunschweig" { "DLR in Braunschweig" }
					"."
				}))

				(thing("MOOdle", "/moodle", true, MOODLE_IMG_SRC, html! {
					"Advanced cow visualization tool, with a 3D pasture simulation written in WebGL. Made with "
					(person(Person::Noa))
					" and "
					(person(Person::Alexis))
					"."
				}))

				(thing("GDPR", "/gdpr", true, GDPR_IMG_SRC, html! {
					"Interactive GDPR presentation "
					(person(Person::Noa))
					" and I made in English class in highschool, which emulates a Windows 7 desktop."
				}))

				(thing("LLN '24", "https://github.com/obiwac/lln-gamejam-2024", false, LLN24_IMG_SRC, html! {
					"Submission for the 2024 Louvain-li-Nux gamejam. Written with "
					(person(Person::Piwy))
					" in Go with a custom WebGPU engine. You play a day in the life of "
					(person(Person::Alexis))
					"."
				}))

				(thing("LLN '23", "https://github.com/obiwac/lln-gamejam-2023", false, LLN23_IMG_SRC, html! {
					"Submission for the 2023 Louvain-li-Nux gamejam. aka "
					(person(Person::Alexis))
					" and "
					(person(Person::Aless))
					" and I's first foray into Vulkan and Rust, aka Obamatriangle."
				}))

				(thing("LLN '22", "https://github.com/obiwac/lln-gamejam-2022", false, LLN22_IMG_SRC, html! {
					"Submission for the 2022 Louvain-li-Nux gamejam, made with "
					(person(Person::Alexis))
					". Pure C11. Pure X11. Pure 7/11."
				}))

				// TODO BATMAN here
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
		a.go-back href="/" {
			(include_static_unsafe!("/icons/back.svg"))
			p { "Main page" }
		}
		.explanation-container {
			.explanation #article {
				header role="banner" {
					h1 { (title) }
				}
				main role="main" {
					(descr)
				}
			}
			aside.exhibit {
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

#[get("/bfm")]
fn bfm() -> Markup {
	explanation_page("Big F'ing Matrix 🌉", BFM_IMG_SRC, html! {
		p {
			"BFM (aka. Big F***ing Matrix) is a FEM/FEA C library with Python bindings and 3D visualization tool. I wrote this with "
			(person(Person::Alex))
			" as our final project for the "
			a.link href="https://perso.uclouvain.be/vincent.legat/zouLab/epl1110.php" { "LEPL1110" }
			" course at uni."
		}
		p {
			"I recently got around to implementing "
			a.link href="https://git@github.com/obiwac/bfm/pulls/1" { "web exporting" }
			" so that you can embed simulation visualizations in a website. You can orbit/pan by left/right clicking, and you can zoom in and out by scrolling."
		}
		p {
			"I have plans to extend this more and use it as an educational tool (complemented by video tutorials). Stay tuned!!"
		}
		.socials {
			(social("Source code", "https://github.com/obiwac/bfm", include_static_unsafe!("/icons/gh.svg")))
		}
	}, html! {
		iframe title="Classical bridge simulation visualization" src="/public/bfm/index.html" loading="lazy";
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

		script #moodle-vert-shader type="x-shader/x-vertex" { (include_static_unsafe!("/moodle/vert.glsl")) }
		script #moodle-frag-shader type="x-shader/x-fragment" { (include_static_unsafe!("/moodle/frag.glsl")) }

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

#[get("/karwa")]
fn karwa() -> Markup {
	explanation_page("KARWa 🧮", KARWA_IMG_SRC, html! {
		p {
			"Francophone algorithmics contest, standing for \"Kompétition d'Algorithmique Régionale Wallonne\". Jointly organized by Louvain-li-Nux (in Louvain-la-Neuve) and CPUMons (in Mons). Created in 2022 by "
			(person(Person::Alex))
			" and I after happening to be in the same train as the Mons team on the way back from the "
			a.link href="https://nwerc.eu/" { "NWERC" }
			" algorithmics contest in Delft. The name was inspired by a legendary karaoke we had in Eindhoven a month prior."
		}
		p { "The first edition was in 2023 and teams had 2 hours and 30 minutes to solve as many problems as possible." }
		p { "It was successful enough to organize again in 2024, and had similar modalities." }
		p { "To the right is a promotional visual I made for the 2024 edition which was played on the screens in the halls of the engineering faculty." }
		.socials {
			(social("KARWa '23", "https://github.com/karwa-org/karwa2023", include_static_unsafe!("//icons/gh.svg")))
			(social("KARWa '24", "https://github.com/karwa-org/karwa2024", include_static_unsafe!("//icons/gh.svg")))
			(social("Website", "https://alexisenglebert.github.io/", include_static_unsafe!("/icons/link.svg")))
		}
	}, html! {
		video loop controls {
			source src="/public/karwa/promo.mp4#t=1" type="video/mp4";
			"Video playback is not supported by your browser."
		}
	})
}

#[get("/graphic-design")]
fn graphic_design() -> Markup {
	explanation_page("Graphic design 🎨", DESIGN_IMG_SRC, html! {
		p {
			"I like creating posters for various student events, especially related to my "
			a.link href="https://en.wikipedia.org/wiki/Theme-based_shared_flat_(kot-%C3%A0-projet)" { "KAP" }
			" ("
			a.link href="https://louvainlinux.org" { "Louvain-li-Nux" }
			")."
		}
		p {
			"To the right are a collection of some of the posters I've made in a scrollable gallery. I don't consider myself to have that much experience in graphic design, so this is mostly all for fun."
		}
		p {
			"Blender was the tool used for most of these (❤️)."
		}
		p {
			"Some of them have animated equivalents which are displayed on our social media profiles."
		}
		.socials {
			(social("@louvainlinux", "https://instagram.com/louvainlinux", include_static_unsafe!("//icons/instagram.svg")))
			(social("Website", "https://louvainlinux.org", include_static_unsafe!("//icons/link.svg")))
		}
	}, html! {
		.image-grid {
				// Posters are ordered by creation date.

				img style="grid-area: gimp" alt="GIMP course poster" src="/public/graphic-design/gimp.webp";
				img style="grid-area: conf" alt="Private life conference poster" src="/public/graphic-design/private-life-conference.webp";
				img style="grid-area: gp22" alt="Geekparty '22 poster" src="/public/graphic-design/gp22.webp";
				img style="grid-area: karwa23" alt="KARWa '23 poster" src="/public/graphic-design/karwa23.webp";
				img style="grid-area: banquet23" alt="Banquet SINFO '23 poster" src="/public/graphic-design/banquet23.webp";
				img style="grid-area: gp23" alt="Geekparty '23 poster" src="/public/graphic-design/gp23.webp";
				img style="grid-area: trilogie" alt="\"Trilogie\" poster" src="/public/graphic-design/trilogie.webp";
				img style="grid-area: gj24" alt="Gamejam '24 poster" src="/public/graphic-design/gj24.webp";
				img style="grid-area: karwa24" alt="KARWa '24 poster" src="/public/graphic-design/karwa24.webp";
		}
	})
}

// server stuff

#[launch]
fn rocket() -> _ {
	let rocket = rocket::build();

	rocket
		.mount("/", routes![index, mcpy, moodle, gdpr, bfm, karwa, graphic_design])
		.mount("/public", FileServer::from(relative!("/public")))
}
