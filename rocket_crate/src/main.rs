#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::content::RawHtml;
use rocket::response::{self, Redirect};

// Define a struct to represent the form data
#[derive(FromForm)]
struct UserForm {
    name: String,
    age: u32,
}

// Define a route handler for the root URL ("/")
#[get("/")]
fn index(flash: Option<FlashMessage>) -> RawHtml<String> {
    let error_message = flash
        .map(|msg| msg.message().to_string())
        .unwrap_or_default();
    // Render the HTML form with optional error message
    let html = format!(
        r#"
        <html>
            <head>
                <title>Form Example</title>
            </head>
            <body>
                <h1>Submit User Data</h1>
                <p style="color: red;">{}</p>
                <form method="post" action="/submit">
                    <label for="name">Name:</label>
                    <input type="text" id="name" name="name" required>
                    <br>
                    <label for="age">Age:</label>
                    <input type="number" id="age" name="age" required>
                    <br>
                    <input type="submit" value="Submit">
                </form>
            </body>
        </html>
    "#,
        error_message
    );

    // Wrap the HTML string in the `RawHtml` type and return it as the response
    RawHtml(html.to_string())
}

// Define a route handler for form submission
#[post("/submit", data = "<user_form>")]
fn submit(user_form: Form<UserForm>) -> Result<String, rocket::response::Flash<Redirect>> {
    // Access the form data from the `user_form` parameter
    let name = &user_form.name;
    let age = user_form.age;

    // Validate the form input
    if age < 18 {
        // If the input is invalid, return an error with a flash message
        let error_msg = format!("Invalid age: {}", age);
        let flash = response::Flash::error(Redirect::to("/"), error_msg);
        return Err(flash);
    }

    // Process the valid form input
    let response = format!("Submitted data: Name - {}, Age - {}", name, age);
    Ok(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit])
}
