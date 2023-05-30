#[macro_use]
extern crate rocket;

use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct User {
    name: String,
}

// Define a route handler for the "/hello/<name>" URL pattern
#[get("/hello/<name>")]
fn hello(name: String) -> Template {
    // Create a User instance
    let user = User { name };
    // Render the "hello" template with the provided name as context
    Template::render("hello", &user)
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
