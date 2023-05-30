#[macro_use]
extern crate rocket;

use rocket::response::content::RawHtml;

struct User {
    name: String,
}

// Define a route handler for the "/hello/<name>" URL pattern
#[get("/hello/<name>")]
fn hello(name: String) -> RawHtml<String> {
    // Create a User instance
    let user = User { name };

    // Generate an HTML string with the user data
    let html = format!(
        "<html>
            <head>
                <title>Dynamic Web Page</title>
            </head>
            <body>
                <h1>Welcome, {}!</h1>
            </body>
        </html>",
        user.name
    );

    RawHtml(html)
}

// Launch the Rocket web application
#[launch]
fn rocket() -> _ {
    rocket::build()
        // Mount the defined routes to the root URL
        .mount("/", routes![hello])
}
