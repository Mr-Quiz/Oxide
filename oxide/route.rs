extern crate oxide_core;

use oxide_core::model::Component;

#[derive(Debug)]
pub struct Route {
  pub component: Option<Component>,
  pub path: String,
}

impl Route {}