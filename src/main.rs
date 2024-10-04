#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
// #![feature(decl_macro)]

#[macro_use] extern crate rocket;
extern crate maud;
extern crate css_minify;
extern crate pulldown_cmark;
extern crate ammonia;

use rocket::fs::FileServer;
use crate::common::relative;
use crate::projects::index;
use crate::project_pages::{mcpy, moodle, gdpr, bfm, karwa, graphic_design, x_compositing_wm, _24hvelo, batman};

mod common;
mod social;
mod person;
mod projects;
mod project_pages;
mod base;

// server stuff

#[launch]
fn rocket() -> _ {
	let rocket = rocket::build();

	rocket
		.mount("/", routes![index, mcpy, moodle, gdpr, bfm, karwa, graphic_design, x_compositing_wm, _24hvelo, batman])
		.mount("/public", FileServer::from(relative!("/public")))
}
