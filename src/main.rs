#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! I was built with Rocket 0.5!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
