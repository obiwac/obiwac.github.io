use maud::{html, Markup, PreEscaped};

use crate::base::base;
use crate::common::{include_static, include_static_unsafe, relative};
use crate::index::{
	BATMAN_IMG_SRC, BFM_IMG_SRC, DESIGN_IMG_SRC, GDPR_IMG_SRC, KARWA_IMG_SRC, MCPY_IMG_SRC, MOODLE_IMG_SRC, X_IMG_SRC,
	_24H_VELO_IMG_SRC,
};
use crate::person::{person, Person};
use crate::social::social;

fn explanation_page(title: &'static str, img_src: &'static str, descr: Markup, exhibit: Markup) -> Markup {
	let schema = format!(
		r#"{{
		"@context": "http://schema.org",
		"@type": "Article",
		"@id": "{{#}}article",
		"name": "{}",
		"author": "Aymeric Wibo",
		"image": "{}"
	}}"#,
		title, img_src
	);

	// TODO Would be nice to have the main-page descriptions as descriptions.

	let description = format!("Project explanation page for \"{}\"", title);

	base(title, &description, PreEscaped(&schema), html! {
		a.go-back href="/" {
			(include_static!("/icons/back.svg"))
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
pub fn mcpy() -> Markup {
	explanation_page(
		"MCPY ⛏️",
		MCPY_IMG_SRC,
		html! {
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
		},
		html! {
			iframe title="Drakeerv's port of MCPY to the browser" src="https://drakeerv.github.io/js-minecraft-clone/episodes/episode-11/index.html" loading="lazy" {}
		},
	)
}

#[get("/bfm")]
pub fn bfm() -> Markup {
	explanation_page(
		"Big F'ing Matrix 🌉",
		BFM_IMG_SRC,
		html! {
			p {
				"BFM (aka. Big Fucking Matrix) is a FEM/FEA C library with Python bindings and 3D visualization tool. I wrote this with "
				(person(Person::Alex))
				" as our final project for the "
				a.link href="https://perso.uclouvain.be/vincent.legat/zouLab/epl1110.php" { "LEPL1110" }
				" course at uni."
			}
			p {
				"I recently got around to implementing "
				a.link href="https://git@github.com/obiwac/bfm/pull/1" { "web exporting" }
				" so that you can embed simulation visualizations in a website. You can orbit/pan by left/right clicking, and you can zoom in and out by scrolling."
			}
			p {
				"I have plans to extend this more and use it as an educational tool (complemented by video tutorials). Stay tuned!!"
			}
			.socials {
				(social("Source code", "https://github.com/obiwac/bfm", include_static!("/icons/gh.svg")))
			}
		},
		html! {
			iframe title="Classical bridge simulation visualization" src="/public/bfm/index.html" loading="lazy" {}
		},
	)
}

#[get("/moodle")]
pub fn moodle() -> Markup {
	explanation_page(
		"MOOdle 🐮",
		MOODLE_IMG_SRC,
		html! {
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
				(social("Source code", "https://github.com/novati0n/moodle", include_static!("/icons/gh.svg")))
				(social("Full version", "https://moodle.novation.dev", include_static!("/icons/link.svg")))
			}
		},
		html! {
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

			script #moodle-vert-shader type="x-shader/x-vertex" { (include_static!("/moodle/vert.glsl")) }
			script #moodle-frag-shader type="x-shader/x-fragment" { (include_static!("/moodle/frag.glsl")) }

			// models

			script src="/public/moodle/models/paturage.js" defer {}
			script src="/public/moodle/models/holstein.js" defer {}
			script src="/public/moodle/models/jersey.js" defer {}
			script src="/public/moodle/models/bbb.js" defer {}

			// actual paturage

			canvas #paturage title="A herd of cows having the time of their lives... in captivity" width="800px" height="500px" onclick="paturage.click()" {}
			script src="/public/moodle/paturage.js" defer {}
		},
	)
}

#[get("/gdpr")]
pub fn gdpr() -> Markup {
	explanation_page(
		"GDPR 🤓",
		GDPR_IMG_SRC,
		html! {
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
				(social("Source code", "https://github.com/novati0n/gdpr-presentation", include_static!("//icons/gh.svg")))
				(social("Full version", "https://novation.dev/GDPR-presentation", include_static!("/icons/link.svg")))
			}
		},
		html! {
			iframe title="The GDPR presentation in question" src="https://novation.dev/GDPR-presentation" loading="lazy" {}
		},
	)
}

#[get("/karwa")]
pub fn karwa() -> Markup {
	explanation_page(
		"KARWa 🧮",
		KARWA_IMG_SRC,
		html! {
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
				(social("KARWa '23", "https://github.com/karwa-org/karwa2023", include_static!("//icons/gh.svg")))
				(social("KARWa '24", "https://github.com/karwa-org/karwa2024", include_static!("//icons/gh.svg")))
				(social("Website", "https://alexisenglebert.github.io/", include_static!("/icons/link.svg")))
			}
		},
		html! {
			video loop controls {
				source src="/public/karwa/promo.mp4#t=1" type="video/mp4";
				"Video playback is not supported by your browser."
			}
		},
	)
}

#[get("/graphic-design")]
pub fn graphic_design() -> Markup {
	explanation_page(
		"Graphic design 🎨",
		DESIGN_IMG_SRC,
		html! {
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
				(social("@louvainlinux", "https://instagram.com/louvainlinux", include_static!("//icons/instagram.svg")))
				(social("Website", "https://louvainlinux.org", include_static!("//icons/link.svg")))
			}
		},
		html! {
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
		},
	)
}

#[get("/x-compositing-wm")]
pub fn x_compositing_wm() -> Markup {
	explanation_page(
		"X Compositing WM 🪟",
		X_IMG_SRC,
		html! {
			p {
				"Super simple compositing window manager for X11 written in C with Xlib and OpenGL (through GLX). Initially this was for prototyping the "
				code { "aquabsd.alps.wm" }
				" device for aquaBSD to, well, manage windows. The point was for it to be a minimal viable example of a compositing window manager."
			}
			p {
				"I got to visit the "
				a.link href="https://www.dlr.de/de/das-dlr/standorte-und-bueros/braunschweig" { "DLR in Braunschweig" }
				" who based a WM on this one to project onto a large spherical screen for use in a helicopter simulator on this code. On the right is a photo of my friend "
				(person(Person::Aditya))
				" attempting to fly it."
			}
			p {
				"Braunschweig and Hanover, which we also stayed at, are both very nice cities. I recommend visiting them if you're in the area."
			}
			.socials {
				(social("Source code", "https://github.com/obiwac/x-compositing-wm", include_static!("//icons/gh.svg")))
			}
		},
		html! {
			img alt="Aditya in the DLR helicopter simulator" src="/public/x-compositing-wm/dlr.jpg";
		},
	)
}

#[get("/24hvelo")]
pub fn _24hvelo() -> Markup {
	explanation_page(
		"24h Vélo 🚲",
		_24H_VELO_IMG_SRC,
		html! {
			p {
				"During the "
				a.link href="https://24heureslln.be" { "24h Vélo de Louvain-la-Neuve" }
				", I built a folkloric bike with "
				(person(Person::Aditya))
				", "
				(person(Person::Piwy))
				", and "
				(person(Person::Alexis))
				" for my "
				a.link href="https://en.wikipedia.org/wiki/Theme-based_shared_flat_(kot-%C3%A0-projet)" { "KAP" }
				" ("
				a.link href="https://louvainlinux.org" { "Louvain-li-Nux" }
				") and I wrote visualization software with "
				(person(Person::Alexis))
				" for a giant 250K EUR screen on the Grand' Place (where the biggest of the 7 concurrent concerts take place). Once they were all done, I played "
				a.link href="https://supertuxkart.net/Main_Page" { "SuperTuxKart" }
				" on it, which is certainly the most expensive gaming monitor I've ever played on."
			}
			p {
				"This page is very much a work in progress, and I've been meaning to make a video on all we did during the event. One day I'll get around to it, hopefully before next year's edition 😊"
			}
			p {
				"The photo on the right is the bike after a rainy night left outside."
			}
			p {
				"I'd also like to host the visualization software itself here."
			}
			.socials {
				(social("Screen source code", "https://github.com/obiwac/24h-lln-screen", include_static!("/icons/gh.svg")))
			}
		},
		html! {
			img alt="The folkloric bike" src="/public/24hvelo/bike.jpg";
		},
	)
}

#[get("/batman")]
pub fn batman() -> Markup {
	explanation_page(
		"B.A.T.M.A.N. 🦇",
		BATMAN_IMG_SRC,
		html! {
			p {
				"As my 2023 GSoC project, I ported the implementation of the B.A.T.M.A.N. mesh routing protocol ("
				code {
					"batman-adv"
				}
				") to FreeBSD."
			}
			p {
				"I gave a talk about this at BSDCan 2024, which was recorded and is embedded here."
				"The slides are also embedded on this page below the video, so you can follow along if you're so inclined (they are made with "
				a.link href="https://marp.app/" { "Marp" }
				" which is actually awesome 💙)."
			}
			.socials {
				(social("Source code", "https://github.com/obiwac/freebsd-gsoc", include_static!("/icons/gh.svg")))
				(social("FreeBSD wiki page", "https://wiki.freebsd.org/SummerOfCode2023Projects/CallingTheBatmanFreeNetworksOnFreeBSD", include_static!("/icons/fbsd.svg")))
				(social("GSoC page", "https://summerofcode.withgoogle.com/archive/2023/projects/9YX3dONN", include_static!("/icons/link.svg")))
			}
		},
		html! {
			.presentation {
				iframe src="https://www.youtube.com/embed/BAVogweBQ8M?list=PLeF8ZihVdpFfct_WnzwObWtj4y9qH3H7X" title="Calling the BATMAN: Free Networks on FreeBSD By: Aymeric Wibo" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen frameborder="0" {}
				/* TODO I need to buy these fonts to be able to use them here!
				style {"
					@font-face {
						font-family: PP Fraktion Mono;
						src: url(¯\\_(ツ)_/¯);
					}

					@font-face {
						font-family: PP Fraktion Neue Machina;
						src: url(¯\\_(ツ)_/¯);
					}
				"}
				*/
				iframe src="/public/batman/presentation.html" allowfullscreen {}
			}
		},
	)
}

pub fn project_page_routes() -> Vec<rocket::Route> {
	routes![
		mcpy,
		bfm,
		moodle,
		gdpr,
		karwa,
		graphic_design,
		x_compositing_wm,
		_24hvelo,
		batman
	]
}
