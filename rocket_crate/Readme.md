# Web Development with Rocket

## Getting Started with Rocket

### Introduction to Rocket, a web framework for Rust

Rocket is a web framework for the Rust programming language that makes it easier to build web applications. It provides a clean and intuitive API, allowing developers to quickly create robust and scalable web services.

Here are some key features and benefits of using Rocket for web development in Rust:

1. Productivity: Rocket aims to maximize developer productivity by providing a simple and expressive syntax. It embraces Rust's strong type system and leverages its powerful macro system to enable concise and readable code.
2. Convention over Configuration: Rocket follows the principle of "convention over configuration." It sets sensible defaults, reducing the need for explicit configuration. This allows developers to focus on writing business logic rather than boilerplate code.
3. Routing and Request Handling: Rocket offers a declarative routing system that allows you to define routes and their corresponding handlers easily. It supports dynamic routing, query parameters, and route guards for authentication and authorization.
4. Templating and Views: Rocket provides built-in support for templating engines like Handlebars and Tera, allowing you to generate dynamic HTML views. This enables you to separate the presentation logic from the business logic of your web application.
5. Form Handling and Validation: Rocket simplifies form handling by providing convenient abstractions for parsing and validating form data. It integrates with popular Rust libraries like serde for seamless serialization and deserialization.
6. Database Integration: Rocket supports seamless integration with various databases through its ecosystem of database crates. You can use Rocket with databases like PostgreSQL, MySQL, SQLite, and more, enabling you to persist and retrieve data in your web applications.
7. Testing and Debugging: Rocket offers a comprehensive testing framework that allows you to write unit tests, integration tests, and end-to-end tests for your web applications. It provides tools for debugging and profiling, helping you identify and fix issues effectively.

### Installation and setup of Rocket

To start working with Rocket, you'll need to follow these steps to install and set it up in your Rust project:

1. Add Rocket as a dependency: Open the `Cargo.toml` file in your project directory and add the Rocket dependency under the `[dependencies]` section. Your `Cargo.toml` file should look like this:

   ```toml
   [dependencies]
   rocket = "0.5.0"
   ```

   This tells Cargo to fetch and use the latest version of the Rocket crate for your project.

1. Fetch dependencies: To download and install the Rocket crate and its dependencies, run the following command in your project directory:

   ```sh
   cargo fetch
   ```

   Cargo will retrieve all the necessary dependencies and make them available for your project.

1. Write your first Rocket application: Open the `src/main.rs` file in your project directory. Replace the existing code with a basic Rocket application. Here's an example of a minimal Rocket application:

   ```rs
   #[macro_use]
   extern crate rocket;

   #[get("/")]
   fn index() -> &'static str {
       "Hello, world!"
   }

   #[launch]
   fn rocket() -> _ {
       rocket::build().mount("/", routes![index])
   }
   ```

   This code defines a simple route handler for the root path ("/") that returns the string "Hello, world!". The `rocket()` function is the entry point for your Rocket application.

1. Build and run your application: Save the changes in `main.rs` and build your application by running the following command:

   ```sh
   cargo build
   ```

   Once the build process is complete, you can run your application with the following command:

   ```sh
   cargo run
   ```

   This will start the Rocket server, and you can access your application in your browser at `http://localhost:8000`.

## Routing and Handling Requests

### Understanding HTTP methods (GET, POST, PUT, DELETE) and routes

In web development, routing refers to the process of mapping incoming HTTP requests to specific actions or handlers in your web application. Routes define the URLs or endpoints that clients can access to interact with your application.

HTTP Methods:

HTTP defines several methods or verbs that indicate the type of action to be performed on a resource. The most commonly used HTTP methods are:

1. GET: The GET method is used to retrieve a resource from the server. When a client sends a GET request to a specific URL, it expects the server to return the requested resource, such as a web page or JSON data.

1. POST: The POST method is used to send data to the server, typically for creating or updating a resource. When a client sends a POST request, the server processes the data and performs the necessary actions based on the request payload.

1. PUT: The PUT method is used to update a resource on the server. It sends the entire representation of the resource to be updated. If the resource doesn't exist, the server may create it.

1. DELETE: The DELETE method is used to delete a resource on the server. When a client sends a DELETE request to a specific URL, the server removes the corresponding resource.

Routes:

Routes define the URL patterns that clients can access to interact with your web application. In Rocket, you can define routes using attributes and associate them with specific request handlers.

### Creating route handlers in Rocket

Route handlers are functions that define the behavior and response for a specific route or URL endpoint. These handlers are responsible for processing incoming requests and generating appropriate responses.

To create a route handler in Rocket, follow these steps:

1. Declare the handler function: Define a function that will handle the desired route. This function can have parameters to extract information from the incoming request, such as path parameters, query parameters, or request bodies.

1. Add the appropriate attribute: Use Rocket's attributes to specify the HTTP method and route pattern associated with the handler function. Rocket provides attributes like `#[get("/")], #[post("/")], #[put("/")], #[delete("/")]`, and more.

1. Define the handler logic: Write the logic inside the handler function to process the request and generate the desired response. This can include database operations, calling external APIs, rendering templates, or any other business logic specific to your application.

Here's an example that demonstrates creating route handlers in Rocket:

```rs
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
```

In this example:

- The `index` function handles a GET request to the root URL ("/") and returns the string "Hello, World!" as the response.
- The `get_user` function handles a GET request to the "/users/&lt;id&gt;" endpoint, where `<id>` is a dynamic path parameter. It takes the `id` as a parameter and returns a formatted string.
- The `create_user` function handles a POST request to the "/users" endpoint. It takes the `user_data` as a parameter, which represents the request body, and returns a formatted string.
- The `update_user` function handles a PUT request to the "/users/&lt;id&gt;" endpoint. It takes the `id` and `user_data` as parameters and returns a formatted string.
- The `delete_user` function handles a DELETE request to the "/users/&lt;id&gt;" endpoint. It takes the `id` as a parameter and returns a formatted string.

With these route handlers defined, Rocket will handle incoming requests and route them to the appropriate functions based on the requested route and HTTP method.

### Implementing request handling logic

When handling requests in Rocket, you can access various components of the request, such as path parameters, query parameters, headers, and request bodies. Additionally, you can generate appropriate responses to send back to the client.

```rs
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
```

In this example:

- The `delay` function demonstrates handling an asynchronous request using the `async` keyword. It introduces an artificial delay using `tokio::time::sleep` to simulate a slow response.
- The `Authorization` struct and `protected_route` function showcase implementing request guarding or authentication.
- The `FromRequest` trait is implemented for `Authorization` to allow Rocket to extract the authorization token from the request headers. The `from_request` function checks if the token starts with "Bearer " and removes the prefix if present. It returns an `Outcome::Success` with an `Authorization` instance containing the extracted token if successful. Otherwise, it returns an `Outcome::Failure` with an unauthorized status code.
- The `protected_route` function is a route handler for the "/protected" URL pattern that requires authorization. It takes an `auth` parameter of type `Authorization`, indicating that this route requires a valid authorization token. It returns a `status::Custom` response with a success status code and a JSON payload indicating access granted with the token.
- The `not_found` function acts as a catcher-all for the 404 status code. If a route is not found, this function is invoked and returns a static string "404 - Not Found".
- Finally, the `rocket` function is the entry point of the Rocket application. It builds a Rocket instance using `rocket::build()`, mounts the defined routes (delay, protected_route) to the root URL ("/"), and registers the `not_found` catcher to handle 404 errors.

## Templating and Views

### Introduction to templating and views in web development

In web development, templating refers to the practice of separating the structure of a web page from its content. Templating allows you to define reusable components and dynamically generate web pages by filling in the content with data.

The purpose of using templates is to improve code organization, promote code reuse, and simplify the process of generating dynamic HTML pages. Templates provide a convenient way to separate the presentation layer from the underlying logic.

Views, in the context of web development, are the representation of data and logic within a specific template. Views contain the information necessary to render a template with the appropriate data values.

Here's an overview of how templating and views work in web development:

1. Template Engines:
   Template engines are libraries or frameworks that provide tools and syntax for defining templates and rendering them with data. They typically support variables, conditionals, loops, and other control structures.

1. Template Files:
   Templates are usually stored in separate files, often with a specific file extension (e.g., .html, .ejs, .hbs). These files contain the HTML structure with placeholders or tags where dynamic data will be inserted.

1. Placeholder Tags:
   Within the template files, placeholder tags are used to mark the locations where dynamic content will be injected. These tags are specific to the template engine being used.

1. View Data:
   Views hold the data that will be passed into the template for rendering. The data can come from various sources, such as a database, API, or user input. Views are typically represented as objects or dictionaries.

1. Rendering:
   Rendering refers to the process of combining the template with the view data to generate the final HTML output. The template engine takes the template file, replaces the placeholder tags with the actual data values from the view, and produces the rendered HTML.

1. Dynamic Content:
   By dynamically populating the template with different view data, you can generate personalized and context-specific content for each user or situation. This allows for the creation of dynamic web pages that adapt to the changing data and requirements.

Popular template engines in web development include Handlebars, EJS (Embedded JavaScript), Jinja2, and Django Templates, among others. Each template engine has its own syntax and features, but they all follow the general principles outlined above.

By utilizing templating and views in web development, you can separate the concerns of data and presentation, making your code more maintainable, modular, and reusable.

### Using Rocket 's built-in templating engine

Rocket provides a powerful templating engine called `Tera` that you can use to render dynamic views in your web application. Tera is a flexible and feature-rich template engine that supports variables, conditionals, loops, macros, and more.

To use the `Tera` template engine in your Rocket application, follow these steps:

1. Add the necessary dependencies: In your `Cargo.toml` file, add the `rocket_dyn_templates` dependency:

   ```toml
   [dependencies.rocket_dyn_templates]
   version = "=0.1.0-rc.3"
   features = ["tera"]
   ```

2. Import the necessary modules: In your Rust file, import the required modules:

   ```rs
   #[macro_use]
   extern crate rocket;

   use rocket_dyn_templates::{Template, context};
   ```

3. Define your templates: Create your template files with the desired HTML structure and placeholders for dynamic content. Place these template files in a directory called `templates` at the root of your project.

   For example, let's create a template file named `hello.tera` with the following content:

   ```html
   <!DOCTYPE html>
   <html>
     <head>
       <title>Hello, {{ name }}!</title>
     </head>
     <body>
       <h1>Hello, {{ name }}!</h1>
     </body>
   </html>
   ```

4. Define a route handler: Create a route handler in Rocket to handle the request and render the template. Here's an example that renders the `hello.tera` template:

   ```rs
   // Define a route handler for the "/hello/<name>" URL pattern
   #[get("/hello/<name>")]
   fn hello(name: String) -> Template {
       // Render the "hello" template with the provided name as context
       Template::render("hello", context! { name: &name })
   }
   ```

5. Mount the template engine: In the `rocket()` function, mount the `Tera` template engine by adding the following line:

   ```rs
   // Launch the Rocket web application
   #[launch]
   fn rocket() -> _ {
       rocket::build()
           // Attach the Template fairing to enable template support
           .attach(Template::fairing())
           // Mount the defined routes to the root URL
           .mount("/", routes![hello])
   }
   ```

With these steps, you have integrated Rocket's built-in templating engine into your Rust web application.

When a user accesses the `/hello/<name>` route, the `hello` route handler will be invoked. It creates a `Context` object, inserts the `name` parameter into the context, and then renders the `hello` template with the context data. The rendered HTML is returned as the response.

The `Template::fairing()` line in the `rocket()` function attaches the `Tera` template engine as a fairing to your Rocket application, enabling the rendering of templates.

In the context of the Rocket web framework in Rust, a "Template fairing" refers to a component or middleware that enables template support within the application. Fairings in Rocket are used to modify or enhance the application's behavior globally, applying to all routes and requests.

The Template fairing specifically allows the application to work with templates, which are files containing dynamic content that can be rendered and populated with data. Templates are commonly used for generating HTML pages or other types of dynamically generated content.

By attaching the Template fairing using the `.attach(Template::fairing())` method in the Rocket application setup, the application gains the ability to render and use templates. This fairing integrates a templating engine into the Rocket framework, allowing the application to load, render, and serve template files, making it easier to generate dynamic content and respond to requests with dynamically generated HTML or other formats.
