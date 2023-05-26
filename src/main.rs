#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
	"Hello world!"
}

#[launch]
fn rocket() -> _ {
	let rocket = rocket::build();
	rocket.mount("/", routes![index])
}
