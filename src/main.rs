#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
// #![feature(decl_macro)]
#![feature(closure_lifetime_binder)]

#[macro_use]
extern crate rocket;
extern crate ammonia;
extern crate css_minify;
extern crate maud;
extern crate pulldown_cmark;

use blog::blog_routes;
use common::relative;
use project_pages::project_page_routes;
use rocket::fs::FileServer;

mod base;
mod blog;
mod common;
mod index;
mod person;
mod project_pages;
mod social;

// server stuff

#[launch]
fn rocket() -> _ {
	let rocket = rocket::build();

	rocket
		.mount("/", routes![index::index])
		.mount("/", project_page_routes())
		.mount("/", blog_routes())
		.mount("/public", FileServer::from(relative!("/public")))
}
