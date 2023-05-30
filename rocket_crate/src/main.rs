#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/login")]
fn login() -> &'static str {
    "Login endpoint"
}

#[put("/users/<id>")]
fn update_user(id: usize) -> String {
    format!("Updating user with ID: {}", id)
}

#[delete("/users/<id>")]
fn delete_user(id: usize) -> String {
    format!("Deleting user with ID: {}", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, login, update_user, delete_user])
}
