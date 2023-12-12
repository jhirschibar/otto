// Import necessary packages
#[macro_use] extern crate rocket;

// Define the route handler for `/test`
#[get("/test")]
fn test() -> &'static str {
    "This is a test endpoint"
}

// Define the main function to start the Rocket server with the `/test` route
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![test])
}