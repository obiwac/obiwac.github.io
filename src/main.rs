#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
// #![feature(decl_macro)]
#![feature(closure_lifetime_binder)]

#[macro_use] extern crate rocket;
extern crate maud;
extern crate css_minify;
extern crate pulldown_cmark;
extern crate ammonia;

use blog::blog_routes;
use rocket::fs::FileServer;
use common::relative;
use project_pages::project_page_routes;

mod common;
mod social;
mod person;
mod index;
mod project_pages;
mod base;
mod blog;

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
