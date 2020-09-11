#[get("/")]
fn index() -> &'static str {
    "Hello"
}

/// Launches the server on the standard port.
/// 
/// # Panics 
/// Panics when server failes to launch and prints error.
pub fn launch_server() {
    let error = rocket::ignite().mount("/", routes![index]).launch();
    panic!(error);
}
