#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::response::content::RawHtml;

// Define a struct to represent the form data
#[derive(FromForm)]
struct UserForm {
    name: String,
    age: u32,
}

// Define a route handler for the root URL ("/")
#[get("/")]
fn index() -> RawHtml<String> {
    // Render the HTML form
    let html = r#"
        <html>
            <head>
                <title>Form Example</title>
            </head>
            <body>
                <h1>Submit User Data</h1>
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
    "#;

    // Wrap the HTML string in the `RawHtml` type and return it as the response
    RawHtml(html.to_string())
}

// Define a route handler for form submission
#[post("/submit", data = "<user_form>")]
fn submit(user_form: Form<UserForm>) -> String {
    // Access the form data from the `user_form` parameter
    let name = &user_form.name;
    let age = user_form.age;

    format!("Submitted data: Name - {}, Age - {}", name, age)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit])
}
