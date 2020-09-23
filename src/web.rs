use vial;

vial::routes! {
  GET "/" => index;
}

fn index(req: vial::Request) -> vial::Response {
  vial::Response::from_asset("html/index.html")
}

pub fn launch_server(port: u16) {
  let address = format!("{}{}", "localhost:", port);
  vial::asset_dir!("./static");
  vial::run_with_banner!("raptr startet at {}", address).unwrap();
}