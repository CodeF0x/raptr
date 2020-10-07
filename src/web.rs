use crate::io;
use vial::{Request, Response};

vial::routes! {
  GET "/" => index;
  GET "/new" => new;
  POST "/new" => submit_file;
  GET "/edit" => list_files;
  GET "/edit/*name" => edit_single_file;
  POST "/edit/*name" => update_file;
}

/// GET / - Shows index site.
fn index(_req: Request) -> Response {
    vial::Response::from_asset("html/index.html")
}

/// GET /new - Shows inputs to create new file.
fn new(_req: Request) -> Response {
    vial::Response::from_asset("html/new.html")    
}

/// POST /new - Submits data to create new file.
fn submit_file(req: Request) -> Response {
    let filename = req.form("filename").unwrap();
    let markdown = req.form("markdown").unwrap();
    match io::write_markdown_to_draft(filename, markdown) {
        Ok(_) => Response::from_asset("html/success.html"),
        Err(err) => Response::from_text(format!("Error while saving draft: {:?}", err)),
    }
}

/// GET /edit - Shows all files in draft directory.
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

/// GET /edit/*name - Shows page to edit single file.
fn edit_single_file(req: Request) -> Response {
    let file_name = req.arg("name").unwrap();
    let file_content = io::read_file(file_name);
    Response::from_body(
        vial::asset::to_string("/html/edit_file.html").unwrap()
        .replace("{{ draft_text }}", &file_content.unwrap())
        .replace("{{ file_name }}", file_name))
}

/// POST /edit/*name - Submits data to update single file.
fn update_file(req: Request) -> Response {
    let file_name = req.arg("name").unwrap();
    let file_content = req.form("markdown").unwrap();
    match io::write_markdown_to_draft(file_name, file_content) {
        Ok(_) => Response::from_asset("html/success.html"),
        Err(err) => Response::from_text(format!("Error while saving draft: {:?}", err)),
    }
}

/// Launches server.
pub fn launch_server(port: u16) {
    let address = format!("{}{}", "localhost:", port);
    vial::asset_dir!("./static");
    vial::run_with_banner!("raptr startet at {}", address).unwrap();
}
