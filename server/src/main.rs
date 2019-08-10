#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "HUGGOOOOOOSSSS)"
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![index])
        .launch();
}
