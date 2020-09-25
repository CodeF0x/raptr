use vial::{Response, Request};
use crate::io;

vial::routes! {
  GET "/" => index;
  GET "/new" => new;
  POST "/new" => new;
}

/// / route.
fn index(_req: Request) -> vial::Response {
  vial::Response::from_asset("html/index.html")
}

/// /new route.
fn new(req: Request) -> vial::Response {
  if req.method() == "GET" {
    vial::Response::from_asset("html/new.html")
  } else {
    let filename = req.form("filename").unwrap();
    let markdown = req.form("markdown").unwrap();
    match io::write_markdown_to_draft(filename, markdown) {
      Ok(_) => Response::from_asset("html/sucess.html"),
      Err(_err) => Response::from_text("Could not save draft, see console"),
    }
  }
}

/// Launches server.
pub fn launch_server(port: u16) {
  let address = format!("{}{}", "localhost:", port);
  vial::asset_dir!("./static");
  vial::run_with_banner!("raptr startet at {}", address).unwrap();
}