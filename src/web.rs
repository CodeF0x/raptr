use vial;

vial::routes! {
  GET "/" => index;
}

fn index(req: vial::Request) -> vial::Response {
  vial::Response::from_asset("html/index.html")
}

pub fn launch_server() {
  vial::asset_dir!("./static");
  vial::run!().unwrap();
}