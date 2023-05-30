#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket::serde::json::{json, Value};
use rocket::tokio::time::{sleep, Duration};

// Define a route handler for the "/delay/<seconds>" URL pattern
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await; // Asynchronously wait for the specified duration
    format!("Delayed response for {} seconds", seconds) // Format a response string indicating the delay
}

// Define a struct to represent authorization information
#[derive(Debug)]
struct Authorization {
    token: String,
}

// Implement the FromRequest trait to extract authorization token from request headers
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorization {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Get the "Authorization" header value
        let token = req.headers().get_one("Authorization").unwrap_or_default(); 
        if token.starts_with("Bearer ") {
            // Remove the "Bearer " prefix from the token to get the actual token
            let token = token.strip_prefix("Bearer ").unwrap_or(token); 
            return request::Outcome::Success(Authorization {
                // Create an Authorization instance with the extracted token
                token: token.to_string(), 
            });
        }
        // Return an unauthorized status if the token is invalid
        request::Outcome::Failure((Status::Unauthorized, ())) 
    }
}

// Define a route handler for the "/protected" URL pattern that requires authorization
#[get("/protected")]
fn protected_route(auth: Authorization) -> status::Custom<Value> {
    status::Custom(
        // Use a success status code
        Status::Ok, 
        json!({
            "message": "Access granted",
            // Include the extracted token in the JSON response
            "token": auth.token 
        }),
    )
}

// Define a catcher for the 404 status code
#[catch(404)]
fn not_found() -> &'static str {
    "404 - Not Found" // Return a static string indicating the resource was not found
}

// Launch the Rocket web application
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![delay, protected_route]) // Mount the defined routes to the root URL
        .register("/", catchers![not_found]) // Register the not_found catcher for handling 404 errors
}
