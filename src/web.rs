use vial;

vial::routes! {
  GET "/" => index;
  GET "/new" => new;
  POST "/new" => new;
}

fn index(req: vial::Request) -> vial::Response {
  vial::Response::from_asset("html/index.html")
}

fn new(req: vial::Request) -> vial::Response {
  if req.method() == "GET" {

    vial::Response::from_asset("html/new.html")
  } else {
    vial::Response::from_text(req.form("markdown").unwrap())
  }
}

pub fn launch_server(port: u16) {
  let address = format!("{}{}", "localhost:", port);
  vial::asset_dir!("./static");
  vial::run_with_banner!("raptr startet at {}", address).unwrap();
}