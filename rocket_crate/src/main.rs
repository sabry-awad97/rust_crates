// Import the Rocket framework
#[macro_use]
extern crate rocket;

// Define a route handler for the root URL "/"
#[get("/")]
fn index() -> &'static str {
    "Hello, World!" // Return a static string as the response
}

// Define a route handler for the "/users/<id>" URL pattern
#[get("/users/<id>")]
fn get_user(id: usize) -> String {
    format!("Fetching user with ID: {}", id) // Format a response string with the provided ID
}

// Define a route handler for the "/users" URL pattern with a POST method
#[post("/users", data = "<user_data>")]
fn create_user(user_data: String) -> String {
    format!("Creating user with data: {}", user_data) // Format a response string with the provided user data
}

// Define a route handler for the "/users/<id>" URL pattern with a PUT method
#[put("/users/<id>", data = "<user_data>")]
fn update_user(id: usize, user_data: String) -> String {
    format!("Updating user with ID {} and data: {}", id, user_data) // Format a response string with the provided ID and user data
}

// Define a route handler for the "/users/<id>" URL pattern with a DELETE method
#[delete("/users/<id>")]
fn delete_user(id: usize) -> String {
    format!("Deleting user with ID: {}", id) // Format a response string with the provided ID
}

// Launch the Rocket web application
#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![index, get_user, create_user, update_user, delete_user], // Mount the defined routes to the root URL
    )
}
