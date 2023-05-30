#[macro_use]
extern crate rocket;

use rocket::{form::Form, response::content::RawHtml};

#[derive(FromForm)]
struct User {
    name: String,
    email: String,
}

#[get("/")]
fn index() -> RawHtml<String> {
    // Generate an HTML string with the user data
    let html = format!(
        r#"<!DOCTYPE html>
            <html>
            <head>
                <title>Dynamic Web Page</title>
            </head>
            <body>
                <form action="/submit" method="post">
                    <label for="name">Name:</label>
                    <input type="text" id="name" name="name" required />
            
                    <label for="email">Email:</label>
                    <input type="email" id="email" name="email" required />
            
                    <input type="submit" value="Submit" />
                </form>
            </body>
            </html>"#,
    );

    RawHtml(html)
}

#[post("/submit", data = "<form_data>")]
fn submit_form(form_data: Form<User>) -> String {
    // Access form data using form_data
    // Perform validation and process the submitted data

    format!(
        "Hello, {}! Your email ({}) has been submitted.",
        form_data.name, form_data.email
    )
}

// Launch the Rocket web application
#[launch]
fn rocket() -> _ {
    rocket::build()
        // Mount the defined routes to the root URL
        .mount("/", routes![index, submit_form])
}
