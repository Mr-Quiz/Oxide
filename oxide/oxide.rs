use crate::settings::Settings;
use crate::route::Route;
use oxide_core::file_parser::CargoFile;
use oxide_core::file_parser::FileParser;
use oxide_core::model::Component;


#[derive(Debug)]
pub struct Oxide {
  pub router: Vec<Route>,
  pub settings: Settings,
  pub current_route: String,
  pub toml_file: CargoFile,
  pub components: Vec<Component>,
}

impl Oxide {
  pub fn new(height: i16, width: i16) -> Oxide {
    return Oxide {
      router: vec![],
      settings: Settings::new(height, width),
      current_route: "/".to_string(),
      toml_file: CargoFile::new(),
      components: vec![],
    };
  }

  pub fn set_router(&mut self, router: Vec<Route>) {
    self.router = router;
  }

  pub fn get_route(self, path: &str) -> Option<Route> {
    let mut route = self.router.into_iter().filter(|route| route.path == path);
    return route.next();
  }

  pub fn search_for_component(&mut self) {}
}