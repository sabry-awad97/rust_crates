#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};

// Define a route handler for the "/hello/<name>" URL pattern
#[get("/hello/<name>")]
fn hello(name: String) -> Template {
    // Render the "hello" template with the provided name as context
    Template::render("hello", context! { name })
}

// Launch the Rocket web application
#[launch]
fn rocket() -> _ {
    rocket::build()
        // Attach the Template fairing to enable template support
        .attach(Template::fairing())
        // Mount the defined routes to the root URL
        .mount("/", routes![hello])
}
