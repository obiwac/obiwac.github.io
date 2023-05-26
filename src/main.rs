#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;

extern crate maud;
use maud::{html, Markup};

#[get("/")]
fn index() -> Markup {
	html! {
		h1 { "Header" }
		p { "Paragraph" }
	}
}

#[launch]
fn rocket() -> _ {
	let rocket = rocket::build();
	rocket.mount("/", routes![index])
}
