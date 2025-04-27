use maud::{html, Markup, PreEscaped};

use crate::base::base;
use crate::blog::BLOGS;
use crate::common::{include_static, include_static_unsafe, relative};
use crate::person::{person, Person};
use crate::social::social;

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

// Imagemagick command to resize image to 400x250: `convert -resize 400x250 input.png output.png`

pub const AQUABSD_IMG_SRC: &str = "/public/thumbnails/aquabsd-small.png";
pub const MCPY_IMG_SRC: &str = "/public/thumbnails/mcpy-small.png";
pub const BFM_IMG_SRC: &str = "/public/thumbnails/bfm-small.png";
pub const KARWA_IMG_SRC: &str = "/public/thumbnails/karwa-small.png";
pub const MOODLE_IMG_SRC: &str = "/public/thumbnails/moodle-small.png";
pub const GDPR_IMG_SRC: &str = "/public/thumbnails/gdpr-small.png";
pub const LLN24_IMG_SRC: &str = "/public/thumbnails/lln24-small.png";
pub const LLN23_IMG_SRC: &str = "/public/thumbnails/lln23-small.png";
pub const LLN22_IMG_SRC: &str = "/public/thumbnails/lln22-small.png";
pub const X_IMG_SRC: &str = "/public/thumbnails/x-small.png";
pub const _24H_VELO_IMG_SRC: &str = "/public/thumbnails/24hvelo-small.png";
pub const DESIGN_IMG_SRC: &str = "/public/thumbnails/graphic-design-small.webp";
pub const BATMAN_IMG_SRC: &str = "/public/thumbnails/batman-small.webp";

fn projects() -> Markup {
	html! {
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

			(thing("B.A.T.M.A.N. on FreeBSD", "/batman", false, BATMAN_IMG_SRC, html! {
				"Port of the B.A.T.M.A.N. mesh routing protocol to FreeBSD. Initially written as a GSoC project."
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
		}
	}
}

fn articles() -> Markup {
	let entries: Vec<Markup> = BLOGS.iter().map(|blog| blog.render_entry()).collect();

	html! {
		@for entry in entries {
			(entry)
			hr;
		}
	}
}

#[get("/")]
pub fn index() -> Markup {
	base(
		"Aymeric Wibo",
		"Personal website for Aymeric Wibo",
		include_static!("/schema/me.json"),
		html! {
			.page-container {
				header role="banner" {
					.section-container {
						center {
							h1 { "Hey! 👋" }
						}
						.socials {
							(social("awibo", "https://www.linkedin.com/in/awibo", include_static!("/icons/linkedin.svg")))
							(social("@obiwac", "https://github.com/obiwac", include_static!("/icons/gh.svg")))
							(social("me@obiw.ac", "mailto:me@obiw.ac", include_static!("/icons/email.svg")))
							(social("obiwac@freebsd.org", "mailto:obiwac@freebsd.org", include_static!("/icons/fbsd.svg")))
							(social("obiwac", "https://youtube.com/obiwac", include_static!("/icons/youtube.svg")))
							(social("obiwac", "https://discord.com/users/305047157197504522", include_static!("/icons/discord.svg")))
							(social("Webring", "http://fuz.su", include_static!("/icons/bell.svg")))
						}
					}
				}
				main role="main" {
					.section-container {
						p {
							"My name is "
							strong { "Aymeric Wibo" }
							" (aka "
							strong { "obiwac" }
							"). I'm a Belgian open-source enthusiast who likes dogs and beer 🍺 Here are some of my bigger projects - those which have a "
							span.inline-svg {
								(include_static!("/icons/magic.svg"))
							}
							" next to their name are interactive experiences."
						}
						p style="margin:0" {
							"My interests programming-wise lie mostly in operating systems and graphics programming, but I'm also a huge public transport nerd."
						}
					}
					.all-my-homies-hate-margin-collapsing {
						input #projects-tab-input type="radio" name="tab" checked;
						input #articles-tab-input type="radio" name="tab";
						.tabs {
							label #projects-tab-label for="projects-tab-input" { "Projects" }
							label #articles-tab-label for="articles-tab-input" { "Articles" }
						}
						.tab-content {
							#projects-tab .tab {
								(projects())
							}
							#articles-tab .tab {
								(articles())
							}
						}
					}
				}
				footer role="contentinfo" {
					.section-container {
						p {
							"This page was made possible thanks to "
							a.link href="https://rocket.rs" { "Rocket.rs" }
							" and "
							a.link href="https://maud.lambda.xyz" { "Maud" }
							"! Fun fact: this site's source doesn't have a single line of the godforsaken language known as HTML in it. It does have some JS on some pages though (not this one), so count that as an L if you want."
						}
						.socials {
							(social("Source code", "https://github.com/obiwac/obiwac.github.io", include_static!("/icons/gh.svg")))
						}
					}
				}
			}
		},
	)
}
