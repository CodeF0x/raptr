/// Launches the server on the standard port.
/// 
/// # Panics 
/// Panics when server failes to launch and prints error.
pub fn launch_server() {
    let error = build_server().launch();
    panic!(error);
}

/// Builds server with confugiration.
fn build_server() -> rocket::Rocket {
    rocket::ignite().mount("/", rocket_contrib::serve::StaticFiles::from("static/html"))
}

#[cfg(test)]
mod tests {
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};
    use super::*;

    #[test]
    fn basic_server_things() {
        let client = Client::new(build_server()).expect("No valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));

        let response = client.get("/nope.html").dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }
}
