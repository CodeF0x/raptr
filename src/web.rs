use crate::io;
use vial::{Request, Response};

vial::routes! {
  GET "/" => index;
  GET "/new" => new;
  POST "/new" => new;
  GET "/edit" => list_files;
  GET "/edit/*name" => edit;
}

/// / route.
fn index(_req: Request) -> Response {
    vial::Response::from_asset("html/index.html")
}

/// /new route.
fn new(req: Request) -> Response {
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

/// GET /edit
fn list_files(_req: Request) -> Response {
    let paths = io::get_files();
    let mut html_string = String::from("<ul>");

    for path in paths {
        let temp_str = format!("<li><a href=\"/edit/{}\">{}</li>", path, path);
        html_string.push_str(&temp_str);
    }
    html_string.push_str("</ul>");
    let string = vial::asset::to_string("/html/edit.html").unwrap().replace("{{ file_list }}", &html_string);
    Response::from_body(string)
}

fn edit(req: Request) -> Response {
    Response::from_text(req.arg("name").unwrap())
}

/// Launches server.
pub fn launch_server(port: u16) {
    let address = format!("{}{}", "localhost:", port);
    vial::asset_dir!("./static");
    vial::run_with_banner!("raptr startet at {}", address).unwrap();
}
